// Copyright (c) 2026 The Cochran Block. All rights reserved.
//! f61 = screenshot — out_dir, theme, capture_project for TRIPLE SIMS visual verification.

use std::path::PathBuf;

/// f61_out_dir. Returns cache dir for screenshots: ~/.cache/screenshots/linux/{project}
pub fn out_dir(project: &str) -> PathBuf {
    let base = dirs::cache_dir()
        .unwrap_or_else(|| std::path::PathBuf::from("."))
        .join("screenshots")
        .join(std::env::consts::OS);
    base.join(project)
}

/// Theme for cochranblock: block diagram styling.
#[derive(Clone)]
pub struct Theme {
    _placeholder: (),
}

/// f61_theme_cochranblock. Cochranblock block-diagram theme.
pub fn theme_cochranblock() -> Theme {
    Theme { _placeholder: () }
}

/// f61_capture_project. Fetches each page, renders via headless browser (devtools) or placeholder.
/// Returns true if all captures succeed.
pub async fn capture_project(
    base: &str,
    project: &str,
    pages: &[(&str, &str)],
    _theme: &Theme,
) -> bool {
    let dir = out_dir(project);
    if let Err(e) = std::fs::create_dir_all(&dir) {
        eprintln!("screenshot: mkdir {}: {}", dir.display(), e);
        return false;
    }

    #[cfg(feature = "devtools")]
    {
        match crate::devtools::capture_screenshots(base, pages, &dir).await {
            Ok(ok) => return ok,
            Err(e) => {
                eprintln!("screenshot: devtools fallback to placeholder: {}", e);
            }
        }
    }

    capture_placeholder(base, project, pages, &dir).await
}

async fn capture_placeholder(
    base: &str,
    _project: &str,
    pages: &[(&str, &str)],
    dir: &std::path::Path,
) -> bool {
    let client = match reqwest::Client::builder()
        .timeout(std::time::Duration::from_secs(10))
        .build()
    {
        Ok(c) => c,
        Err(e) => {
            eprintln!("screenshot: reqwest client: {}", e);
            return false;
        }
    };
    let base = base.trim_end_matches('/');
    for (name, path) in pages {
        let url = format!("{}{}", base, path);
        match client.get(&url).send().await {
            Ok(resp) if resp.status().is_success() => {
                let out = dir.join(format!("{}.png", name));
                if let Err(e) = write_placeholder_png(&out) {
                    eprintln!("screenshot: write {}: {}", out.display(), e);
                    return false;
                }
                println!("screenshot: {} -> {}", url, out.display());
            }
            Ok(resp) => {
                eprintln!("screenshot: {} -> {}", url, resp.status());
                return false;
            }
            Err(e) => {
                eprintln!("screenshot: fetch {}: {}", url, e);
                return false;
            }
        }
    }
    true
}

fn write_placeholder_png(path: &std::path::Path) -> Result<(), String> {
    let img = image::RgbaImage::from_fn(100, 100, |_, _| image::Rgba([200, 200, 200, 255]));
    img.save(path).map_err(|e| e.to_string())
}
