// Copyright (c) 2026 The Cochran Block. All rights reserved.
#![allow(non_camel_case_types, non_snake_case, dead_code, unused_imports)]

use colored::Colorize;
use crate::std_aliases::F374;

pub mod unit;
pub mod integration;
pub mod http;

#[derive(Clone)]
pub struct t24 {
    pub s30: String,
    pub s31: bool,
    pub s32: u64,
    pub s33: Option<String>,
}

/// f52 = print_test_result — PASS/FAIL + name + ms
pub fn f52(p0: &t24) {
    if p0.s31 {
        println!("{}  {}  {}ms", "PASS".green(), p0.s30, p0.s32);
    } else {
        let reason = p0.s33.as_deref().unwrap_or("(no reason)");
        println!("{}  {}  {}ms  {}", "FAIL".red(), p0.s30, p0.s32, reason);
    }
}

/// Assert-style helper: Ok(()) = pass, Err(msg) = fail with reason. No self-licking.
pub fn assert_ok(pass: bool, msg: impl Into<String>) -> Result<(), String> {
    if pass {
        Ok(())
    } else {
        Err(msg.into())
    }
}

/// f30 = run_tests — f49 f50 f51 aggregate
pub async fn f30() -> bool {
    println!("\n=== Unit Tests ===");
    let v0 = unit::f49().await;
    for v1 in &v0 { f52(v1); }
    println!("\n=== Integration Tests ===");
    let v1 = integration::f50().await;
    for v2 in &v1 { f52(v2); }
    println!("\n=== HTTP Tests ===");
    let v2 = http::f51().await;
    for v3 in &v2 { f52(v3); }
    let mut v4 = v0.f374().chain(v1.f374()).chain(v2.f374());
    v4.all(|v5| v5.s31)
}
