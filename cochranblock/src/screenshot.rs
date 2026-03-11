// Copyright (c) 2026 The Cochran Block. All rights reserved.
#![allow(non_camel_case_types, non_snake_case, dead_code, unused_imports)]

//! f53 = screenshot_capture. exopack for TRIPLE SIMS visual verification. Why: Kova forbids unwrap in lib.

use exopack::screenshot;
use std::fs;
use std::path::Path;

use crate::t0;
use crate::web::router;

const COCHRANBLOCK_PAGES: &[(&str, &str)] = &[
    ("index", "/"),
    ("services", "/services"),
    ("products", "/products"),
    ("federal-partners", "/federal-partners"),
    ("about", "/about"),
    ("contact", "/contact"),
    ("book", "/book"),
];

/// Copy screenshots from cache to repo screenshots/ for TRIPLE SIMS docs.
fn copy_to_repo() {
    let cache = screenshot::out_dir("cochranblock");
    let repo = Path::new(env!("CARGO_MANIFEST_DIR")).join("screenshots");
    if let Err(e) = fs::create_dir_all(&repo) {
        eprintln!("screenshot: mkdir {}: {}", repo.display(), e);
        return;
    }
    for (name, _) in COCHRANBLOCK_PAGES {
        let src = cache.join(format!("{}.png", name));
        let dst = repo.join(format!("{}.png", name));
        if src.exists() {
            if let Err(e) = fs::copy(&src, &dst) {
                eprintln!("screenshot: copy {} -> {}: {}", src.display(), dst.display(), e);
            }
        }
    }
    println!("Screenshots copied to {}/ (for TRIPLE SIMS)", repo.display());
}

/// f53 = screenshot_capture. Why: TRIPLE SIMS visual verification via exopack.
pub async fn f53() -> bool {
    let p0 = t0 {};
    let v1 = router::f1(p0);
    let v2 = match tokio::net::TcpListener::bind("127.0.0.1:0").await {
        Ok(v) => v,
        Err(e) => {
            eprintln!("screenshot: bind failed: {}", e);
            return false;
        }
    };
    let v3 = match v2.local_addr() {
        Ok(addr) => addr,
        Err(e) => {
            eprintln!("screenshot: local_addr failed: {}", e);
            return false;
        }
    };
    let v4 = format!("http://127.0.0.1:{}", v3.port());
    tokio::spawn(async move { let _ = axum::serve(v2, v1).await; });

    tokio::time::sleep(tokio::time::Duration::from_millis(500)).await;

    let theme = screenshot::theme_cochranblock();
    let ok = screenshot::capture_project(&v4, "cochranblock", COCHRANBLOCK_PAGES, &theme).await;
    if ok {
        copy_to_repo();
    }
    ok
}
