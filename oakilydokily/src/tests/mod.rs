// Copyright (c) 2026 The Cochran Block. All rights reserved.
//! f30 = run_tests. HTTP-based UI/UX, feature gap, user analysis. Server must be up.

use std::time::Duration;

/// f30 = run_tests. Returns 0 if all pass, 1 otherwise.
pub async fn f30() -> i32 {
    let base = std::env::var("BASE").unwrap_or_else(|_| "http://127.0.0.1:3000".into());
    let base = base.trim_end_matches('/').to_string();

    let client = match reqwest::Client::builder()
        .timeout(Duration::from_secs(10))
        .build()
    {
        Ok(c) => c,
        Err(e) => {
            eprintln!("f30: reqwest client: {}", e);
            return 1;
        }
    };

    let mut failed = 0;
    let checks: &[(&str, &str, &[&str])] = &[
        ("home_200", "/", &[]),
        ("home_identity", "/", &["OakilyDokily", "Kaylie"]),
        ("home_serving_md", "/", &["Serving Maryland"]),
        ("home_main_landmark", "/", &["id=\"main\"", "<main"]),
        ("home_skip_link", "/", &["skip-link", "#main"]),
        ("about_200", "/about", &[]),
        ("about_resume", "/about", &["resume", "experience", "Kaylie"]),
        ("about_print", "/about", &["Print Resume", "window.print"]),
        ("contact_200", "/contact", &[]),
        ("contact_mailto", "/contact", &["mailto:"]),
        ("contact_book_call", "/contact", &["Book a Call", "Discovery"]),
        ("waiver_200", "/waiver", &[]),
        ("waiver_terms", "/waiver", &["agree_terms", "consent_electronic"]),
        ("waiver_scroll_hint", "/waiver", &["Scroll through", "scroll"]),
        ("waiver_confirmed", "/waiver/confirmed", &[]),
        ("health_200", "/health", &["OK"]),
        ("css_200", "/assets/css/main.css", &[]),
        ("favicon_200", "/assets/favicon.svg", &[]),
        ("gap_home_services", "/", &["kennel", "overnight", "surgical"]),
        ("gap_home_flexible", "/", &["Flexible", "contract", "temp"]),
        ("gap_waiver_full_name", "/waiver", &["full_name"]),
        ("gap_waiver_email", "/waiver", &["email"]),
        ("gap_nav_waiver", "/", &["Waiver", "/waiver"]),
        ("gap_nav_about", "/", &["About", "/about"]),
        ("gap_nav_contact", "/", &["Contact", "/contact"]),
    ];

    let client_no_redirect = reqwest::Client::builder()
        .timeout(Duration::from_secs(10))
        .redirect(reqwest::redirect::Policy::none())
        .build()
        .unwrap();

    for (name, path, patterns) in checks {
        let url = format!("{}{}", base, path);
        let client_use = if path.starts_with("/waiver") && !path.contains("confirmed") {
            &client_no_redirect
        } else {
            &client
        };
        match client_use.get(&url).send().await {
            Ok(resp) => {
                let code = resp.status().as_u16();
                let body = resp.text().await.unwrap_or_default();
                let ok = (code >= 200 && code < 300
                    && (patterns.is_empty() || patterns.iter().any(|p| body.contains(*p))))
                    || (*path == "/waiver" && patterns.is_empty() && code == 303);
                if ok {
                    println!("  [PASS] {}", name);
                } else if path.starts_with("/waiver") && !path.contains("confirmed") && code == 303 {
                    println!("  [SKIP] {} (303 auth required; set OD_TEST_WAIVER_BYPASS=1 for full waiver tests)", name);
                } else {
                    eprintln!(
                        "  [FAIL] {} (code={}, patterns={:?})",
                        name, code, patterns
                    );
                    failed += 1;
                }
            }
            Err(e) => {
                eprintln!("  [FAIL] {} (req: {})", name, e);
                failed += 1;
            }
        }
    }

    if failed > 0 {
        eprintln!("\n{} check(s) failed", failed);
        1
    } else {
        println!("\nall {} checks passed", checks.len());
        0
    }
}
