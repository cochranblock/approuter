// Copyright (c) 2026 The Cochran Block. All rights reserved.
#![allow(non_camel_case_types, non_snake_case, dead_code, unused_imports)]
//! f30 = run_tests, f49 = unit, f50 = integration, f51 = HTTP

mod http;
mod integration;
mod unit;

use std::time::Instant;

pub struct t24 {
    pub name: String,
    pub passed: bool,
    pub duration_ms: u64,
    pub message: Option<String>,
}

pub async fn f30() -> bool {
    let mut results = Vec::new();

    for t in unit::f49() {
        results.push(t);
    }
    for t in integration::f50().await {
        results.push(t);
    }
    for t in http::f51().await {
        results.push(t);
    }

    let passed = results.iter().filter(|r| r.passed).count();
    let total = results.len();

    for r in &results {
        let status = if r.passed { "PASS" } else { "FAIL" };
        let color = if r.passed { "\x1b[32m" } else { "\x1b[31m" };
        let reset = "\x1b[0m";
        let msg = r.message.as_deref().unwrap_or("");
        println!(
            "{}  {} {}{} {}ms {}",
            color, status, reset, r.name, r.duration_ms, msg
        );
    }

    println!("\n{} / {} passed", passed, total);
    passed == total
}
