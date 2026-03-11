// Copyright (c) 2026 The Cochran Block. All rights reserved.
//! DevTools: headless browser console check via CDP. Catches JS errors, syntax errors, console.error.
//! f63 = capture_screenshots. Real browser screenshots including WASM canvas.
//!
//! When Chrome is not installed, the fetcher downloads Chromium to ~/.cache/chromiumoxide.
//! On Linux, install libnspr4 and libnss3 (e.g. `apt install libnspr4 libnss3`) for the fetched binary to run.

use std::path::Path;
use std::sync::Arc;
use std::time::Duration;

use chromiumoxide::page::ScreenshotParams;
use chromiumoxide::cdp::browser_protocol::page::CaptureScreenshotFormat;

/// Build BrowserConfig. Uses fetcher to download Chromium when auto-detect fails.
async fn browser_config() -> Result<chromiumoxide::BrowserConfig, String> {
    let builder = chromiumoxide::BrowserConfig::builder();
    match builder.build() {
        Ok(c) => return Ok(c),
        Err(e) if e.contains("Could not auto detect") => {}
        Err(e) => return Err(format!("devtools config: {}", e)),
    }
    let dir = dirs::cache_dir()
        .unwrap_or_else(|| std::path::PathBuf::from("."))
        .join("chromiumoxide");
    std::fs::create_dir_all(&dir).map_err(|e| format!("mkdir {}: {}", dir.display(), e))?;
    let fetcher = chromiumoxide::fetcher::BrowserFetcher::new(
        chromiumoxide::fetcher::BrowserFetcherOptions::builder()
            .with_path(&dir)
            .build()
            .map_err(|e| format!("fetcher options: {}", e))?,
    );
    let info = fetcher.fetch().await.map_err(|e| format!("fetcher: {}", e))?;
    chromiumoxide::BrowserConfig::builder()
        .chrome_executable(info.executable_path)
        .build()
        .map_err(|e| format!("devtools config: {}", e))
}

/// f62 = check_console_errors. Navigates to each URL, collects console errors. Returns errors or empty.
pub async fn check_console_errors(base: &str, paths: &[&str]) -> Result<Vec<String>, String> {
    let base = base.trim_end_matches('/');
    let config = browser_config().await?;

    let (mut browser, mut handler) =
        chromiumoxide::Browser::launch(config).await.map_err(|e| format!("devtools launch: {}", e))?;

    let handle = tokio::spawn(async move {
        while futures::StreamExt::next(&mut handler).await.is_some() {}
    });

    let mut all_errors = Vec::new();
    for path in paths {
        let url = format!("{}{}", base, path);
        if let Ok(errors) = check_one_url(&browser, &url).await {
            for e in errors {
                all_errors.push(format!("{}: {}", url, e));
            }
        }
    }

    let _ = browser.close().await;
    handle.abort();

    Ok(all_errors)
}

/// f63 = capture_screenshots. Launches headless Chromium, navigates to each URL, waits for render (WASM),
/// saves PNG to out_dir. Returns true if all succeed.
pub async fn capture_screenshots(
    base: &str,
    pages: &[(&str, &str)],
    out_dir: &Path,
) -> Result<bool, String> {
    let base = base.trim_end_matches('/');
    if let Err(e) = std::fs::create_dir_all(out_dir) {
        return Err(format!("screenshot mkdir {}: {}", out_dir.display(), e));
    }
    let config = browser_config().await?;

    let (mut browser, mut handler) =
        chromiumoxide::Browser::launch(config).await.map_err(|e| format!("devtools launch: {}", e))?;

    let handle = tokio::spawn(async move {
        while futures::StreamExt::next(&mut handler).await.is_some() {}
    });

    let mut ok = true;
    for (name, path) in pages {
        let url = format!("{}{}", base, path);
        let out = out_dir.join(format!("{}.png", name));
        let wait_secs = if *name == "mural" { 8 } else { 3 };
        match capture_one(&browser, &url, &out, wait_secs).await {
            Ok(()) => println!("screenshot: {} -> {}", url, out.display()),
            Err(e) => {
                eprintln!("screenshot: {} -> {}", url, e);
                ok = false;
            }
        }
    }

    let _ = browser.close().await;
    handle.abort();

    Ok(ok)
}

async fn capture_one(
    browser: &chromiumoxide::Browser,
    url: &str,
    out: &Path,
    wait_secs: u64,
) -> Result<(), String> {
    let page = browser
        .new_page("about:blank")
        .await
        .map_err(|e| format!("new_page: {}", e))?;

    let _ = page.goto(url).await.map_err(|e| format!("goto: {}", e))?;
    tokio::time::sleep(Duration::from_secs(wait_secs)).await;

    page.save_screenshot(
        ScreenshotParams::builder()
            .format(CaptureScreenshotFormat::Png)
            .full_page(true)
            .build(),
        out,
    )
    .await
    .map_err(|e| format!("save_screenshot: {}", e))?;

    Ok(())
}

async fn check_one_url(
    browser: &chromiumoxide::Browser,
    url: &str,
) -> Result<Vec<String>, String> {
    let page = browser
        .new_page("about:blank")
        .await
        .map_err(|e| format!("new_page: {}", e))?;

    let (tx, mut rx) = tokio::sync::mpsc::unbounded_channel::<String>();
    let tx = Arc::new(tx);

    let mut console_events = page
        .event_listener::<chromiumoxide::cdp::js_protocol::runtime::EventConsoleApiCalled>()
        .await
        .map_err(|e| format!("event_listener: {}", e))?;

    use chromiumoxide::cdp::js_protocol::runtime::ConsoleApiCalledType;
    let tx_clone = tx.clone();
    let logs_handle = tokio::spawn(async move {
        while let Some(event) = futures::StreamExt::next(&mut console_events).await {
            if event.r#type == ConsoleApiCalledType::Error || event.r#type == ConsoleApiCalledType::Warning {
                let msg: String = event
                    .args
                    .iter()
                    .filter_map(|a| a.value.as_ref())
                    .map(|v| v.to_string())
                    .collect::<Vec<_>>()
                    .join(" ");
                if !msg.is_empty() {
                    let _ = tx_clone.send(msg);
                }
            }
        }
    });

    let mut log_events = page
        .event_listener::<chromiumoxide::cdp::browser_protocol::log::EventEntryAdded>()
        .await
        .map_err(|e| format!("log listener: {}", e))?;

    let _ = page.execute(chromiumoxide::cdp::browser_protocol::log::EnableParams::default()).await;

    let tx_log = tx.clone();
    let log_handle = tokio::spawn(async move {
        while let Some(event) = futures::StreamExt::next(&mut log_events).await {
            use chromiumoxide::cdp::browser_protocol::log::LogEntryLevel;
            if event.entry.level == LogEntryLevel::Error || event.entry.level == LogEntryLevel::Warning {
                let msg = event.entry.text.clone();
                if !msg.is_empty() {
                    let _ = tx_log.send(msg);
                }
            }
        }
    });

    let _ = page.goto(url).await;

    tokio::time::sleep(Duration::from_secs(2)).await;

    drop(tx);
    logs_handle.abort();
    log_handle.abort();

    let mut errors = Vec::new();
    while let Ok(msg) = rx.try_recv() {
        errors.push(msg);
    }

    Ok(errors)
}
