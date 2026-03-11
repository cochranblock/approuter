// Copyright (c) 2026 The Cochran Block. All rights reserved.
//! Screenshot helpers for TRIPLE SIMS. Uses exopack devtools (chromiumoxide) for real browser screenshots including WASM.

use exopack::devtools;
use exopack::screenshot;

const PAGES: &[(&str, &str)] = &[
    ("home", "/"),
    ("about", "/about"),
    ("contact", "/contact"),
    ("waiver", "/waiver"),
    ("waiver_confirmed", "/waiver/confirmed"),
];

const CONSOLE_PATHS: &[&str] = &["/", "/about", "/contact", "/waiver", "/waiver/confirmed"];

fn is_benign_console_msg(msg: &str) -> bool {
    let s = msg.to_lowercase();
    s.contains("registred plugins") || s.contains("old version of the plugin")
        || (s.contains("webgl") && s.contains("deprecated"))
        || s.contains("gpu stall") || s.contains("gl_close_path")
        || s.contains("groupmarkernotset")
        || (s.contains("failed to load resource") && s.contains("404"))
}

/// f62 = check_console_errors. Headless Chromium visits each path, collects console.error/warning. Returns errors or empty.
pub async fn f62() -> Vec<String> {
    let base = std::env::var("BASE").unwrap_or_else(|_| "http://127.0.0.1:3000".into());
    match devtools::check_console_errors(base.as_str(), CONSOLE_PATHS).await {
        Ok(errors) => errors
            .into_iter()
            .filter(|e| !is_benign_console_msg(e))
            .collect(),
        Err(e) => vec![format!("devtools: {}", e)],
    }
}

/// f53 = capture_oakilydokily. Headless Chromium screenshots to ~/.cache/screenshots/linux/oakilydokily.
pub async fn f53() -> bool {
    let base = std::env::var("BASE").unwrap_or_else(|_| "http://127.0.0.1:3000".into());
    let dir = screenshot::out_dir("oakilydokily");
    match devtools::capture_screenshots(base.as_str(), PAGES, &dir).await {
        Ok(ok) => ok,
        Err(e) => {
            eprintln!("screenshot: {}", e);
            false
        }
    }
}
