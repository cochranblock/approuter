// Copyright (c) 2026 The Cochran Block. All rights reserved.
//! Baked-in demo: replicates every intended use of Kova with zero user input.
//! Run programmatically to iterate through dev cycles without human interaction.

use std::path::Path;
use std::process::Command;
use std::time::Duration;

/// Run the full baked demo: every CLI subcommand + every HTTP endpoint.
/// Replicates intended usage as if the user were using it. No recording.
/// Returns Ok(()) if all steps pass.
pub async fn run_baked_demo(
    kova_bin: &Path,
    home: &Path,
    port: u16,
) -> Result<(), String> {
    let env_home = home.to_string_lossy().to_string();

    // 1–5. CLI: bootstrap, prompts, model list, recent, c2 nodes (spawn_blocking to avoid blocking runtime)
    let run = |args: Vec<&'static str>| {
        let bin = kova_bin.to_path_buf();
        let home = env_home.clone();
        tokio::task::spawn_blocking(move || run_kova(&bin, &home, &args))
    };
    run(vec!["bootstrap"]).await.map_err(|e| e.to_string())??;
    run(vec!["prompts"]).await.map_err(|e| e.to_string())??;
    run(vec!["model", "list"]).await.map_err(|e| e.to_string())??;
    run(vec!["recent", "--minutes", "60"]).await.map_err(|e| e.to_string())??;
    run(vec!["c2", "nodes"]).await.map_err(|e| e.to_string())??;

    // 6. HTTP: spawn serve
    let demo_dir = home.join(".kova").join("demos");
    let projects_root = home.to_string_lossy().to_string();
    let mut child = Command::new(kova_bin)
        .env("HOME", &env_home)
        .env("KOVA_PROJECT", &projects_root)
        .env("KOVA_PROJECTS_ROOT", &projects_root)
        .env("KOVA_BIND", format!("127.0.0.1:{}", port))
        .env("KOVA_DEMO_DIR", &demo_dir)
        .current_dir(home)
        .args(["serve"])
        .stdout(std::process::Stdio::null())
        .stderr(std::process::Stdio::null())
        .spawn()
        .map_err(|e| format!("spawn serve: {}", e))?;

    tokio::time::sleep(Duration::from_secs(2)).await;

    let client = crate::interface::http_client().map_err(|e| e.to_string())?;
    let base = format!("http://127.0.0.1:{}", port);

    // 7. GET / (web client)
    get_ok(&client, &format!("{}/", base)).await?;

    // 8. GET /api/status (baseline latency check)
    let start = std::time::Instant::now();
    get_ok(&client, &format!("{}/api/status", base)).await?;
    let elapsed = start.elapsed();
    if elapsed.as_secs_f64() > 2.0 {
        return Err(format!("api/status latency {}s exceeds 2s baseline", elapsed.as_secs_f64()));
    }

    // 9. GET /api/project
    get_ok(&client, &format!("{}/api/project", base)).await?;

    // 10. GET /api/projects
    get_ok(&client, &format!("{}/api/projects", base)).await?;

    // 11. GET /api/prompts
    get_ok(&client, &format!("{}/api/prompts", base)).await?;

    // 12. GET /api/backlog
    get_ok(&client, &format!("{}/api/backlog", base)).await?;

    // 13. GET /context/recent
    get_ok(&client, &format!("{}/context/recent", base)).await?;

    // 14. GET /build/presets
    get_ok(&client, &format!("{}/build/presets", base)).await?;

    // 15. POST /api/intent (FullPipeline — accepted even without models)
    let intent_body = serde_json::json!({
        "s0": {"FullPipeline": null},
        "s1": "add a test function",
        "s2": [{"MustNotBreakTests": null}, {"MayUseGpu": null}]
    });
    post_json_ok(&client, &format!("{}/api/intent", base), &intent_body).await?;

    // 16. POST /api/backlog (add entry)
    let backlog_body = serde_json::json!({
        "intent": "full-pipeline",
        "project": null
    });
    post_json_ok(&client, &format!("{}/api/backlog", base), &backlog_body).await?;

    // 17. POST /api/diff
    let diff_body = serde_json::json!({
        "hint": "lib.rs",
        "new_content": "fn demo() {}"
    });
    let _ = client
        .post(format!("{}/api/diff", base))
        .json(&diff_body)
        .timeout(Duration::from_secs(5))
        .send()
        .await;
    // diff may succeed or fail (no project); we don't require 200

    // 18. POST /api/demo/record
    let demo_body = serde_json::json!({
        "name": "baked-demo",
        "source": "baked",
        "actions": [
            {"kind": "get", "path": "/api/status"},
            {"kind": "get", "path": "/api/projects"},
            {"kind": "post", "path": "/api/intent", "body": "FullPipeline"}
        ],
        "started_at": "0"
    });
    post_json_ok(&client, &format!("{}/api/demo/record", base), &demo_body).await?;

    let _ = child.kill();
    let _ = child.wait();

    // Verify demo artifact written
    let demo_file = demo_dir.join("baked-demo.json");
    if !demo_file.exists() {
        return Err(format!("demo artifact not found: {:?}", demo_file));
    }

    Ok(())
}

fn run_kova(bin: &Path, home: &str, args: &[&str]) -> Result<(), String> {
    let out = Command::new(bin)
        .env("HOME", home)
        .env("KOVA_PROJECT", home)
        .env("KOVA_PROJECTS_ROOT", home)
        .args(args)
        .output()
        .map_err(|e| format!("spawn: {}", e))?;
    if !out.status.success() {
        return Err(format!(
            "kova {} failed: {}",
            args.join(" "),
            String::from_utf8_lossy(&out.stderr)
        ));
    }
    Ok(())
}

async fn get_ok(client: &reqwest::Client, url: &str) -> Result<(), String> {
    let ok = client
        .get(url)
        .timeout(Duration::from_secs(5))
        .send()
        .await
        .map_err(|e| e.to_string())?
        .status()
        .is_success();
    if !ok {
        return Err(format!("GET {} failed", url));
    }
    Ok(())
}

async fn post_json_ok(
    client: &reqwest::Client,
    url: &str,
    body: &serde_json::Value,
) -> Result<(), String> {
    let ok = client
        .post(url)
        .json(body)
        .timeout(Duration::from_secs(5))
        .send()
        .await
        .map_err(|e| e.to_string())?
        .status()
        .is_success();
    if !ok {
        return Err(format!("POST {} failed", url));
    }
    Ok(())
}
