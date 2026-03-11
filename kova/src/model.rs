// Copyright (c) 2026 The Cochran Block. All rights reserved.
//! Model install. f77=model_install

/// f77=model_install. Download Qwen2.5-Coder-0.5B-Instruct GGUF to ~/.kova/models/
#[cfg(feature = "inference")]
pub async fn f77() -> anyhow::Result<()> {
    use std::io::Write;

    let filename = "Qwen2.5-Coder-0.5B-Instruct-Q4_K_M.gguf";
    let url = format!(
        "https://huggingface.co/bartowski/Qwen2.5-Coder-0.5B-Instruct-GGUF/resolve/main/{}",
        filename
    );
    let models_dir = crate::models_dir();
    std::fs::create_dir_all(&models_dir)?;
    let dest = models_dir.join(filename);

    if dest.exists() {
        eprintln!("Model already exists: {}", dest.display());
        return Ok(());
    }

    eprintln!("Downloading Qwen2.5-Coder-0.5B-Instruct (~380 MB)...");
    eprintln!("  {}", url);

    let client = reqwest::Client::builder()
        .user_agent("kova/0.1")
        .build()?;
    let mut resp = client.get(url).send().await?;
    resp.error_for_status_ref()?;

    let total = resp.content_length().unwrap_or(0);
    let mut file = std::fs::File::create(&dest)?;
    let mut downloaded: u64 = 0;

    while let Some(chunk) = resp.chunk().await? {
        file.write_all(&chunk)?;
        downloaded += chunk.len() as u64;
        if total > 0 && downloaded % (2 * 1024 * 1024) < chunk.len() as u64 {
            let pct = (downloaded as f64 / total as f64) * 100.0;
            eprint!("\r  {:.1}%", pct);
        }
    }
    eprintln!("\r  Done. {}", dest.display());

    Ok(())
}
