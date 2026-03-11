<!-- Copyright (c) 2026 The Cochran Block. All rights reserved. -->
# Rust Self-Evaluating Test Binary — Comprehensive Architecture Guide
## Two-Binary Model with Real Validation, Mock Infrastructure, and Cross-Language Bootstrap

---

# TABLE OF CONTENTS

1. [Architecture Overview](#1-architecture-overview)
2. [Unit Tests — Real Validation](#2-unit-tests)
3. [UI Tests — Headless Browser via Python Injection](#3-ui-tests)
4. [Integration Tests — Mock Infrastructure](#4-integration-tests)
5. [Mock API Implementation](#5-mock-api-implementation)
6. [Chicken-and-Egg Bootstrap Patterns](#6-chicken-and-egg-bootstrap)
7. [Directory Structure and Crate Recommendations](#7-directory-structure)
8. [Test Report Generation](#8-test-report)

> **Compression notation** from `kova/docs/compression_map.md` and `compression_map_testing.md` is used
> throughout. All compact identifiers expand deterministically to human-readable code.

---

# 1. ARCHITECTURE OVERVIEW

## 1.1 The Two-Binary Model

```
┌─────────────────────────────────────────────────────────────────────┐
│                        CARGO WORKSPACE                              │
│                                                                     │
│  ┌─────────────────────────┐    ┌────────────────────────────────┐  │
│  │   BINARY 1: app         │    │   BINARY 2: test-runner        │  │
│  │   (production)          │    │   (self-evaluating)            │  │
│  │                         │    │                                │  │
│  │  - Axum web server      │    │  - Spawns Binary 1 as child   │  │
│  │  - DNS monitor          │    │  - Spawns mock servers         │  │
│  │  - Auth, crypto, DB     │    │  - Injects Python/shell for   │  │
│  │  - Embedded assets      │    │    UI testing                 │  │
│  │  - ~10MB release        │    │  - Contains ALL test logic    │  │
│  │                         │    │  - Validates real behavior     │  │
│  │  Cargo features:        │    │  - ~25MB (includes test deps) │  │
│  │    default = []         │    │                                │  │
│  │                         │    │  Cargo features:               │  │
│  │                         │    │    test-support = [...]         │  │
│  └─────────────────────────┘    └────────────────────────────────┘  │
│           │                                │                        │
│           │  shared library crate          │                        │
│           └──────────┐  ┌─────────────────┘                        │
│                      │  │                                           │
│              ┌───────┴──┴────────┐                                  │
│              │   LIB: core       │                                  │
│              │                   │                                  │
│              │  All business     │                                  │
│              │  logic lives here │                                  │
│              │  (testable units) │                                  │
│              └───────────────────┘                                  │
└─────────────────────────────────────────────────────────────────────┘
```

**Key insight:** The production binary stays lean. ALL testing infrastructure — mock servers, Python injection, screenshot comparison, test fixtures — lives exclusively in the test binary. The shared library crate exposes the business logic that both binaries consume, making unit testing possible without the full binary.

## 1.2 Test Execution Flow

```
test-runner binary starts
    │
    ├─► Phase 1: UNIT TESTS (f49)
    │   Run against lib crate directly — no processes spawned
    │   Tests: crypto round-trip, password hashing, config parsing,
    │          DNS record building, error mapping, session logic
    │
    ├─► Phase 2: INTEGRATION TESTS (f50)
    │   ├── f32: Spawn mock Cloudflare API (wiremock on random port)
    │   ├── f32: Spawn mock ipify (wiremock on random port)
    │   ├── f31: Spawn app binary as child process
    │   │        (env vars point app at mock URLs)
    │   ├── f33: Wait for app health endpoint
    │   ├── Run tests: setup flow, login, settings, DNS update cycle
    │   └── f45: Kill child processes, cleanup
    │
    ├─► Phase 3: UI TESTS (f51)
    │   ├── f31: Spawn app binary (if not already running)
    │   ├── f41: Inject Python (Playwright) scripts
    │   │        - Navigate pages, click elements, fill forms
    │   │        - Capture screenshots
    │   │        - Return structured JSON results to Rust
    │   ├── f35: Compare screenshots against baselines
    │   └── f45: Cleanup
    │
    └─► Phase 4: REPORT (f52)
        Generate test report with pass/fail, durations, screenshots
```

## 1.3 Why Two Binaries Instead of `cargo test`

Standard `cargo test` runs `#[test]` functions in the same process. This fails for your requirements because:

1. **You can't spawn yourself as a server** — the app binary needs to be running as a real HTTP server to test against. `cargo test` doesn't give you a running server process.

2. **UI tests need a real browser hitting a real server** — not a mock handler, not a test client. A real Chromium instance navigating to `http://localhost:{port}`.

3. **Integration tests need to verify the binary's actual behavior** — not just the library functions. Does the binary actually start? Does it actually listen on the port? Does it actually encrypt the token before writing to disk?

4. **The test binary IS the CI/CD pipeline** — it contains everything needed to validate the application. No external test runner, no Jenkins, no GitHub Actions config. The binary itself knows how to test itself.

The shared lib crate still uses standard `#[cfg(test)]` for pure unit tests. The test-runner binary handles everything that requires process orchestration.

---

# 2. UNIT TESTS — REAL VALIDATION

## 2.1 Philosophy: No Ice Cream Cones

Every unit test must answer: **"If this function is broken, will this test catch it?"**

Anti-patterns (ice cream cones) we explicitly avoid:
```rust
// ❌ ICE CREAM CONE: tests that the function returns without panicking
#[test]
fn test_encrypt() {
    let result = encrypt_token("test", &[0u8; 32]);
    assert!(result.is_ok()); // USELESS — doesn't verify the output is actually encrypted
}

// ❌ ICE CREAM CONE: tests that a struct can be constructed
#[test]
fn test_config() {
    let config = Config::default();
    assert_eq!(config.port, 8080); // USELESS — tests the Default impl, not real behavior
}
```

What we do instead — **property-based validation**:

## 2.2 Crypto Module Tests (m3)

```rust
#[cfg(test)]
mod tests {
    use super::*;

    // REAL TEST: Encrypt-decrypt round-trip preserves data
    #[test]
    fn encrypt_decrypt_roundtrip_preserves_plaintext() {
        let key = derive_key("test-master-key-32-bytes-long!!!").unwrap();
        let original = "cf_api_token_abc123_real_looking_value";

        let encrypted = encrypt_token(original, &key).unwrap();
        let decrypted = decrypt_token(&encrypted, &key).unwrap();

        assert_eq!(decrypted, original, "Round-trip must preserve plaintext exactly");
    }

    // REAL TEST: Encrypted output is NOT the plaintext
    #[test]
    fn encrypted_output_is_not_plaintext() {
        let key = derive_key("test-master-key-32-bytes-long!!!").unwrap();
        let original = "cf_api_token_abc123";

        let encrypted = encrypt_token(original, &key).unwrap();

        // The encrypted bytes must NOT contain the plaintext as a substring
        let plaintext_bytes = original.as_bytes();
        for window in encrypted.windows(plaintext_bytes.len()) {
            assert_ne!(
                window, plaintext_bytes,
                "Plaintext found verbatim in ciphertext — encryption is broken"
            );
        }
    }

    // REAL TEST: Different plaintexts produce different ciphertexts
    #[test]
    fn different_inputs_produce_different_outputs() {
        let key = derive_key("test-master-key-32-bytes-long!!!").unwrap();

        let enc1 = encrypt_token("token_aaa", &key).unwrap();
        let enc2 = encrypt_token("token_bbb", &key).unwrap();

        assert_ne!(enc1, enc2, "Different plaintexts must produce different ciphertexts");
    }

    // REAL TEST: Same plaintext encrypted twice produces different ciphertexts (nonce uniqueness)
    #[test]
    fn same_input_produces_different_ciphertexts_due_to_nonce() {
        let key = derive_key("test-master-key-32-bytes-long!!!").unwrap();
        let original = "same_token_value";

        let enc1 = encrypt_token(original, &key).unwrap();
        let enc2 = encrypt_token(original, &key).unwrap();

        assert_ne!(
            enc1, enc2,
            "Same plaintext encrypted twice must differ (random nonce)"
        );

        // But both must decrypt to the same value
        let dec1 = decrypt_token(&enc1, &key).unwrap();
        let dec2 = decrypt_token(&enc2, &key).unwrap();
        assert_eq!(dec1, dec2);
        assert_eq!(dec1, original);
    }

    // REAL TEST: Wrong key fails to decrypt
    #[test]
    fn wrong_key_fails_decryption() {
        let key1 = derive_key("correct-master-key-value-here!!!").unwrap();
        let key2 = derive_key("wrong-master-key-value-here!!!!!").unwrap();

        let encrypted = encrypt_token("secret_token", &key1).unwrap();
        let result = decrypt_token(&encrypted, &key2);

        assert!(
            result.is_err(),
            "Decryption with wrong key must fail, not return garbage"
        );
    }

    // REAL TEST: Tampered ciphertext fails authentication
    #[test]
    fn tampered_ciphertext_fails_authentication() {
        let key = derive_key("test-master-key-32-bytes-long!!!").unwrap();
        let encrypted = encrypt_token("secret_token", &key).unwrap();

        // Flip one bit in the ciphertext (after the nonce)
        let mut tampered = encrypted.clone();
        if tampered.len() > 13 {
            tampered[13] ^= 0x01;
        }

        let result = decrypt_token(&tampered, &key);
        assert!(
            result.is_err(),
            "AES-GCM must reject tampered ciphertext — authentication tag check failed"
        );
    }

    // REAL TEST: Truncated ciphertext is rejected
    #[test]
    fn truncated_ciphertext_rejected() {
        let key = derive_key("test-master-key-32-bytes-long!!!").unwrap();
        let encrypted = encrypt_token("secret_token", &key).unwrap();

        // Truncate to just the nonce
        let truncated = &encrypted[..12];
        let result = decrypt_token(truncated, &key);
        assert!(result.is_err(), "Truncated data must be rejected");

        // Empty input
        let result = decrypt_token(&[], &key);
        assert!(result.is_err(), "Empty data must be rejected");
    }
}
```

## 2.3 Password Hashing Tests (m2)

```rust
#[cfg(test)]
mod tests {
    use super::*;

    // REAL TEST: Hash is not the password
    #[test]
    fn hash_is_not_plaintext_password() {
        let password = "MySecureP@ssw0rd!";
        let hash = hash_password(password).unwrap();

        assert_ne!(hash, password, "Hash must not equal plaintext");
        assert!(
            !hash.contains(password),
            "Hash must not contain plaintext as substring"
        );
    }

    // REAL TEST: Correct password verifies
    #[test]
    fn correct_password_verifies() {
        let password = "MySecureP@ssw0rd!";
        let hash = hash_password(password).unwrap();

        assert!(
            verify_password(&hash, password).is_ok(),
            "Correct password must verify against its own hash"
        );
    }

    // REAL TEST: Wrong password fails verification
    #[test]
    fn wrong_password_fails_verification() {
        let password = "MySecureP@ssw0rd!";
        let hash = hash_password(password).unwrap();

        let result = verify_password(&hash, "WrongPassword123!");
        assert!(
            result.is_err(),
            "Wrong password must fail verification"
        );
    }

    // REAL TEST: Same password produces different hashes (unique salt)
    #[test]
    fn same_password_different_hashes() {
        let password = "MySecureP@ssw0rd!";
        let hash1 = hash_password(password).unwrap();
        let hash2 = hash_password(password).unwrap();

        assert_ne!(
            hash1, hash2,
            "Same password hashed twice must produce different hashes (random salt)"
        );

        // But both must verify
        assert!(verify_password(&hash1, password).is_ok());
        assert!(verify_password(&hash2, password).is_ok());
    }

    // REAL TEST: Hash meets minimum computational cost
    #[test]
    fn hashing_takes_minimum_time() {
        let start = std::time::Instant::now();
        let _ = hash_password("test_password").unwrap();
        let elapsed = start.elapsed();

        // Argon2id with proper params should take at least 50ms
        // If it's instant, the cost parameters are too low
        assert!(
            elapsed.as_millis() >= 50,
            "Password hashing took only {}ms — cost parameters are too low for security",
            elapsed.as_millis()
        );
    }
}
```

## 2.4 DNS Record Builder Tests (m1)

```rust
#[cfg(test)]
mod tests {
    use super::*;

    // REAL TEST: Cloudflare API request body is correctly structured
    #[test]
    fn dns_update_body_has_required_fields() {
        let body = build_dns_update_body("203.0.113.42", "A", 300, false);
        let json: serde_json::Value = serde_json::from_str(&body).unwrap();

        assert_eq!(json["type"], "A", "Record type must be 'A'");
        assert_eq!(json["content"], "203.0.113.42", "Content must be the IP");
        assert_eq!(json["ttl"], 300, "TTL must match");
        assert_eq!(json["proxied"], false, "Proxied must match");
    }

    // REAL TEST: IPv6 address is accepted for AAAA records
    #[test]
    fn ipv6_address_accepted_for_aaaa() {
        let body = build_dns_update_body("2001:db8::1", "AAAA", 300, false);
        let json: serde_json::Value = serde_json::from_str(&body).unwrap();

        assert_eq!(json["type"], "AAAA");
        assert_eq!(json["content"], "2001:db8::1");
    }

    // REAL TEST: IP change detection works correctly
    #[test]
    fn ip_change_detected() {
        assert!(ip_has_changed("1.2.3.4", "5.6.7.8"));
        assert!(!ip_has_changed("1.2.3.4", "1.2.3.4"));
        assert!(!ip_has_changed("", "1.2.3.4"), "Empty new IP must not trigger update");
        assert!(ip_has_changed("1.2.3.4", ""), "Empty stored IP means first run — must update");
    }

    // REAL TEST: Cloudflare API URL is correctly constructed
    #[test]
    fn cloudflare_url_construction() {
        let url = build_cf_url("zone123", "record456");
        assert_eq!(
            url,
            "https://api.cloudflare.com/client/v4/zones/zone123/dns_records/record456"
        );
    }
}
```

## 2.5 Session Management Tests (m2)

```rust
#[cfg(test)]
mod tests {
    use super::*;

    // REAL TEST: Session ID is valid UUID v4
    #[test]
    fn session_id_is_valid_uuid() {
        let session = Session::new("admin", Duration::from_secs(3600));
        let parsed = uuid::Uuid::parse_str(&session.id);
        assert!(parsed.is_ok(), "Session ID must be valid UUID");
        assert_eq!(parsed.unwrap().get_version_num(), 4, "Must be UUID v4");
    }

    // REAL TEST: Session expiry is in the future
    #[test]
    fn session_expires_in_future() {
        let session = Session::new("admin", Duration::from_secs(3600));
        assert!(
            session.expires_at > chrono::Utc::now(),
            "New session must expire in the future"
        );
    }

    // REAL TEST: Expired session is detected
    #[test]
    fn expired_session_detected() {
        let mut session = Session::new("admin", Duration::from_secs(3600));
        // Manually backdate the expiry
        session.expires_at = chrono::Utc::now() - chrono::Duration::seconds(1);
        assert!(session.is_expired(), "Backdated session must be detected as expired");
    }

    // REAL TEST: Two sessions have different IDs (no collision)
    #[test]
    fn sessions_have_unique_ids() {
        let s1 = Session::new("admin", Duration::from_secs(3600));
        let s2 = Session::new("admin", Duration::from_secs(3600));
        assert_ne!(s1.id, s2.id, "Two sessions must have different IDs");
    }
}
```

## 2.6 Error Mapping Tests (m7)

```rust
#[cfg(test)]
mod tests {
    use super::*;
    use axum::http::StatusCode;

    // REAL TEST: Internal errors don't leak details to client
    #[test]
    fn internal_errors_are_sanitized() {
        let error = AppError::Database("SQLITE_CORRUPT: database disk image is malformed".into());
        let response = error.into_response();

        assert_eq!(response.status(), StatusCode::INTERNAL_SERVER_ERROR);

        // Extract body and verify it does NOT contain the internal message
        let body = response_body_string(response);
        assert!(
            !body.contains("SQLITE_CORRUPT"),
            "Internal error details must not leak to client. Got: {body}"
        );
        assert!(
            !body.contains("malformed"),
            "Internal error details must not leak to client. Got: {body}"
        );
    }

    // REAL TEST: Auth errors return 401, not 500
    #[test]
    fn auth_errors_return_401() {
        let error = AppError::Unauthorized;
        let response = error.into_response();
        assert_eq!(response.status(), StatusCode::UNAUTHORIZED);
    }

    // REAL TEST: Rate limit errors return 429
    #[test]
    fn rate_limit_returns_429() {
        let error = AppError::RateLimited;
        let response = error.into_response();
        assert_eq!(response.status(), StatusCode::TOO_MANY_REQUESTS);
    }
}
```

---

# 3. UI TESTS — HEADLESS BROWSER VIA PYTHON INJECTION

## 3.1 The Chicken-and-Egg Problem

Rust has no mature headless browser automation library comparable to Playwright or Selenium. The options are:
- `fantoccini` — WebDriver client, requires separate chromedriver process, limited API
- `headless_chrome` — Direct CDP, unmaintained, crashes on complex pages
- `thirtyfour` — WebDriver, better than fantoccini but still WebDriver limitations

**The bootstrap solution:** Inject Python with Playwright into the Rust test binary. Python's Playwright is the gold standard for browser automation. The Rust binary orchestrates everything — it spawns the Python process, passes it the test script, and parses the structured JSON results back.

This is not a hack. This is the same pattern used by:
- Chromium's test infrastructure (C++ spawning Python test scripts)
- Android's CTS (Java spawning shell commands for device interaction)
- Every CI/CD system ever (orchestrator language ≠ test language)

## 3.2 Python Injection Infrastructure

```rust
// In the test-runner binary: src/injection/mod.rs

use std::process::Command;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct UiTestResult {
    pub passed: bool,
    pub test_name: String,
    pub duration_ms: u64,
    pub screenshot_path: Option<String>,
    pub error: Option<String>,
    pub assertions: Vec<UiAssertion>,
}

#[derive(Debug, Deserialize)]
pub struct UiAssertion {
    pub description: String,
    pub passed: bool,
    pub expected: String,
    pub actual: String,
}

/// Ω20: Execute a Python script and parse structured JSON output
pub fn inject_python(script: &str) -> Result<Vec<UiTestResult>, TestError> {
    let output = Command::new("python3")
        .arg("-c")
        .arg(script)
        .output()
        .map_err(|e| TestError::Injection(format!("Failed to spawn Python: {e}")))?;

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        return Err(TestError::Injection(format!("Python script failed: {stderr}")));
    }

    let stdout = String::from_utf8_lossy(&output.stdout);

    // Parse the last line as JSON (script may print debug info before)
    let json_line = stdout.lines().last()
        .ok_or(TestError::Injection("No output from Python".into()))?;

    serde_json::from_str(json_line)
        .map_err(|e| TestError::Injection(format!("Failed to parse Python output: {e}")))
}

/// Ω21: Execute a shell command and capture output
pub fn inject_shell(cmd: &str) -> Result<InjectionResult, TestError> {
    let output = Command::new("sh")
        .arg("-c")
        .arg(cmd)
        .output()
        .map_err(|e| TestError::Injection(format!("Shell failed: {e}")))?;

    Ok(InjectionResult {
        exit_code: output.status.code().unwrap_or(-1),
        stdout: String::from_utf8_lossy(&output.stdout).into_owned(),
        stderr: String::from_utf8_lossy(&output.stderr).into_owned(),
    })
}
```

## 3.3 Playwright Test Scripts (Embedded in Rust as String Constants)

```rust
// src/ui_tests/scripts.rs
// Python scripts embedded as Rust string constants — compiled into the test binary

/// Ensure Playwright is installed (run once at test suite start)
pub const SETUP_PLAYWRIGHT: &str = r#"
import subprocess, sys
try:
    from playwright.sync_api import sync_playwright
    print('{"ready": true}')
except ImportError:
    subprocess.check_call([sys.executable, "-m", "pip", "install", "playwright"])
    subprocess.check_call([sys.executable, "-m", "playwright", "install", "chromium"])
    print('{"ready": true}')
"#;

/// Test: Resume page loads and contains expected content
pub fn resume_page_test(base_url: &str) -> String {
    format!(r#"
import json
from playwright.sync_api import sync_playwright

results = []

with sync_playwright() as p:
    browser = p.chromium.launch(headless=True)
    page = browser.new_page(viewport={{"width": 1280, "height": 720}})

    # Navigate to resume page
    page.goto("{base_url}/resume", wait_until="networkidle")

    assertions = []

    # REAL CHECK: Page title contains expected text
    title = page.title()
    assertions.append({{
        "description": "Page title contains 'Michael Cochran'",
        "passed": "Michael Cochran" in title,
        "expected": "Contains 'Michael Cochran'",
        "actual": title
    }})

    # REAL CHECK: Resume content is present (not a blank page)
    body_text = page.inner_text("body")
    assertions.append({{
        "description": "Page body contains professional experience",
        "passed": "USCYBERCOM" in body_text or "Cyber" in body_text,
        "expected": "Contains work history keywords",
        "actual": f"Body length: {{len(body_text)}} chars"
    }})

    # REAL CHECK: Navigation links exist and are clickable
    nav_links = page.query_selector_all("nav a")
    assertions.append({{
        "description": "Navigation has at least 3 links",
        "passed": len(nav_links) >= 3,
        "expected": ">= 3 nav links",
        "actual": str(len(nav_links))
    }})

    # REAL CHECK: No JavaScript errors on page
    errors = []
    page.on("pageerror", lambda e: errors.append(str(e)))
    page.reload(wait_until="networkidle")
    assertions.append({{
        "description": "No JavaScript errors on page",
        "passed": len(errors) == 0,
        "expected": "0 JS errors",
        "actual": f"{{len(errors)}} errors: {{errors[:3]}}"
    }})

    # REAL CHECK: Page responds within acceptable time
    import time
    start = time.time()
    page.goto("{base_url}/resume", wait_until="networkidle")
    load_time = (time.time() - start) * 1000
    assertions.append({{
        "description": "Page loads within 2000ms",
        "passed": load_time < 2000,
        "expected": "< 2000ms",
        "actual": f"{{load_time:.0f}}ms"
    }})

    # Screenshot for visual regression
    screenshot_path = "/tmp/test_resume_page.png"
    page.screenshot(path=screenshot_path, full_page=True)

    all_passed = all(a["passed"] for a in assertions)
    results.append({{
        "passed": all_passed,
        "test_name": "resume_page_content_and_structure",
        "duration_ms": int(load_time),
        "screenshot_path": screenshot_path,
        "error": None if all_passed else "One or more assertions failed",
        "assertions": assertions
    }})

    browser.close()

print(json.dumps(results))
"#)
}

/// Test: Admin login flow — form submission, session creation, redirect
pub fn admin_login_test(base_url: &str, username: &str, password: &str) -> String {
    format!(r#"
import json
from playwright.sync_api import sync_playwright

results = []

with sync_playwright() as p:
    browser = p.chromium.launch(headless=True)
    context = browser.new_context()
    page = context.new_page()

    assertions = []

    # Navigate to login page
    page.goto("{base_url}/login", wait_until="networkidle")

    # REAL CHECK: Login form exists with expected fields
    username_field = page.query_selector('input[name="username"]')
    password_field = page.query_selector('input[name="password"]')
    submit_btn = page.query_selector('button[type="submit"]')

    assertions.append({{
        "description": "Login form has username field",
        "passed": username_field is not None,
        "expected": "input[name=username] exists",
        "actual": "found" if username_field else "NOT FOUND"
    }})
    assertions.append({{
        "description": "Login form has password field",
        "passed": password_field is not None,
        "expected": "input[name=password] exists",
        "actual": "found" if password_field else "NOT FOUND"
    }})

    # Fill and submit the form
    if username_field and password_field and submit_btn:
        page.fill('input[name="username"]', "{username}")
        page.fill('input[name="password"]', "{password}")
        page.click('button[type="submit"]')
        page.wait_for_load_state("networkidle")

        # REAL CHECK: After login, we're redirected to admin page (not back to login)
        current_url = page.url
        assertions.append({{
            "description": "Successful login redirects to /admin",
            "passed": "/admin" in current_url and "/login" not in current_url,
            "expected": "URL contains /admin",
            "actual": current_url
        }})

        # REAL CHECK: Session cookie was set
        cookies = context.cookies()
        session_cookie = [c for c in cookies if c["name"] == "session_id"]
        assertions.append({{
            "description": "Session cookie is set after login",
            "passed": len(session_cookie) > 0,
            "expected": "session_id cookie exists",
            "actual": f"{{len(session_cookie)}} matching cookies"
        }})

        if session_cookie:
            # REAL CHECK: Cookie has HttpOnly flag
            assertions.append({{
                "description": "Session cookie is HttpOnly",
                "passed": session_cookie[0].get("httpOnly", False),
                "expected": "httpOnly=true",
                "actual": str(session_cookie[0].get("httpOnly"))
            }})

            # REAL CHECK: Cookie has SameSite=Strict
            assertions.append({{
                "description": "Session cookie has SameSite=Strict",
                "passed": session_cookie[0].get("sameSite") == "Strict",
                "expected": "sameSite=Strict",
                "actual": str(session_cookie[0].get("sameSite"))
            }})

    # REAL CHECK: Accessing /admin without login redirects to /login
    context2 = browser.new_context()  # fresh context, no cookies
    page2 = context2.new_page()
    page2.goto("{base_url}/admin", wait_until="networkidle")
    assertions.append({{
        "description": "Unauthenticated /admin access redirects to /login",
        "passed": "/login" in page2.url,
        "expected": "Redirected to /login",
        "actual": page2.url
    }})

    screenshot_path = "/tmp/test_admin_login.png"
    page.screenshot(path=screenshot_path)

    all_passed = all(a["passed"] for a in assertions)
    results.append({{
        "passed": all_passed,
        "test_name": "admin_login_flow",
        "duration_ms": 0,
        "screenshot_path": screenshot_path,
        "error": None if all_passed else "One or more assertions failed",
        "assertions": assertions
    }})

    browser.close()

print(json.dumps(results))
"#)
}

/// Test: Wrong password is rejected
pub fn admin_login_rejection_test(base_url: &str) -> String {
    format!(r#"
import json
from playwright.sync_api import sync_playwright

results = []

with sync_playwright() as p:
    browser = p.chromium.launch(headless=True)
    page = browser.new_page()

    assertions = []

    page.goto("{base_url}/login", wait_until="networkidle")
    page.fill('input[name="username"]', "admin")
    page.fill('input[name="password"]', "completely_wrong_password")
    page.click('button[type="submit"]')
    page.wait_for_load_state("networkidle")

    # REAL CHECK: Still on login page (not redirected to admin)
    assertions.append({{
        "description": "Wrong password keeps user on login page",
        "passed": "/login" in page.url or page.url.endswith("/login"),
        "expected": "Remains on /login",
        "actual": page.url
    }})

    # REAL CHECK: Error message is displayed
    body = page.inner_text("body")
    assertions.append({{
        "description": "Error message shown for wrong password",
        "passed": "invalid" in body.lower() or "error" in body.lower() or "incorrect" in body.lower(),
        "expected": "Error message visible",
        "actual": f"Body contains error keywords: {{'invalid' in body.lower()}}"
    }})

    # REAL CHECK: No session cookie set
    cookies = page.context.cookies()
    session_cookies = [c for c in cookies if c["name"] == "session_id"]
    assertions.append({{
        "description": "No session cookie after failed login",
        "passed": len(session_cookies) == 0,
        "expected": "0 session cookies",
        "actual": str(len(session_cookies))
    }})

    all_passed = all(a["passed"] for a in assertions)
    results.append({{
        "passed": all_passed,
        "test_name": "admin_login_rejection",
        "duration_ms": 0,
        "screenshot_path": None,
        "error": None if all_passed else "Assertions failed",
        "assertions": assertions
    }})

    browser.close()

print(json.dumps(results))
"#)
}
```

## 3.4 Visual Regression Testing

```rust
// src/ui_tests/visual.rs

/// f35: Compare screenshot against baseline using pixel diff
/// Uses Python Pillow for image comparison — injected via Ω20
pub fn compare_screenshot(actual_path: &str, baseline_path: &str, threshold: f64) -> Result<bool, TestError> {
    let script = format!(r#"
import json
from PIL import Image
import math

actual = Image.open("{actual_path}")
baseline = Image.open("{baseline_path}")

# Resize if dimensions differ (responsive layout changes)
if actual.size != baseline.size:
    actual = actual.resize(baseline.size)

pixels_actual = list(actual.getdata())
pixels_baseline = list(baseline.getdata())

if len(pixels_actual) != len(pixels_baseline):
    print(json.dumps({{"match": false, "diff_pct": 100.0, "reason": "pixel count mismatch"}}))
else:
    diff_count = 0
    for pa, pb in zip(pixels_actual, pixels_baseline):
        # Compare RGB channels with tolerance (anti-aliasing differences)
        if any(abs(a - b) > 10 for a, b in zip(pa[:3], pb[:3])):
            diff_count += 1

    diff_pct = (diff_count / len(pixels_actual)) * 100
    print(json.dumps({{"match": diff_pct < {threshold}, "diff_pct": round(diff_pct, 2)}}))
"#);

    let result = inject_python(&script)?;
    // Parse the comparison result
    let stdout = result.first()
        .ok_or(TestError::Injection("No comparison result".into()))?;

    Ok(stdout.passed)
}

/// Generate baseline screenshots (run once, commit to repo)
pub fn generate_baselines(base_url: &str) -> Result<(), TestError> {
    let script = format!(r#"
import json
from playwright.sync_api import sync_playwright
import os

os.makedirs("test_baselines", exist_ok=True)

with sync_playwright() as p:
    browser = p.chromium.launch(headless=True)
    page = browser.new_page(viewport={{"width": 1280, "height": 720}})

    pages = [
        ("/", "index"),
        ("/resume", "resume"),
        ("/whitepaper", "whitepaper"),
        ("/innovation", "innovation"),
        ("/login", "login"),
    ]

    for path, name in pages:
        page.goto(f"{base_url}{{path}}", wait_until="networkidle")
        page.screenshot(path=f"test_baselines/{{name}}.png", full_page=True)

    browser.close()

print(json.dumps({{"generated": len(pages)}}))
"#);

    inject_python(&script)?;
    Ok(())
}
```

## 3.5 UI Test Runner (Rust Orchestration)

```rust
// src/ui_tests/runner.rs

pub async fn run_ui_suite(ctx: &TestContext) -> Vec<UiTestResult> {
    let mut all_results = Vec::new();
    let base_url = format!("http://127.0.0.1:{}", ctx.app_port);

    // Ensure Playwright is installed
    tracing::info!("Checking Playwright installation...");
    if let Err(e) = inject_python(SETUP_PLAYWRIGHT) {
        tracing::error!("Playwright setup failed: {e}");
        all_results.push(UiTestResult {
            passed: false,
            test_name: "playwright_setup".into(),
            duration_ms: 0,
            screenshot_path: None,
            error: Some(format!("Playwright not available: {e}")),
            assertions: vec![],
        });
        return all_results;
    }

    // Run each UI test
    let tests: Vec<(&str, String)> = vec![
        ("resume_page", resume_page_test(&base_url)),
        ("admin_login", admin_login_test(&base_url, "admin", &ctx.test_password)),
        ("login_rejection", admin_login_rejection_test(&base_url)),
    ];

    for (name, script) in tests {
        tracing::info!("Running UI test: {name}");
        match inject_python(&script) {
            Ok(results) => all_results.extend(results),
            Err(e) => {
                all_results.push(UiTestResult {
                    passed: false,
                    test_name: name.into(),
                    duration_ms: 0,
                    screenshot_path: None,
                    error: Some(format!("{e}")),
                    assertions: vec![],
                });
            }
        }
    }

    all_results
}
```

---

# 4. INTEGRATION TESTS — MOCK INFRASTRUCTURE

## 4.1 Test Context: The Orchestration Core

```rust
// src/integration/context.rs

use std::process::{Child, Command};
use std::net::TcpListener;
use wiremock::MockServer;

pub struct TestContext {
    pub app_process: Option<Child>,
    pub app_port: u16,
    pub mock_cloudflare: MockServer,
    pub mock_ipify: MockServer,
    pub db_path: String,
    pub test_password: String,
    pub master_key: String,
    pub cf_requests: Arc<Mutex<Vec<ReceivedRequest>>>,
}

#[derive(Debug, Clone)]
pub struct ReceivedRequest {
    pub method: String,
    pub path: String,
    pub body: String,
    pub headers: Vec<(String, String)>,
    pub timestamp: chrono::DateTime<chrono::Utc>,
}

impl TestContext {
    /// Σ11: Full mock environment setup
    pub async fn setup() -> Result<Self, TestError> {
        // Ω22: Find random free ports
        let app_port = find_free_port();
        let temp_dir = tempfile::tempdir()?;
        let db_path = temp_dir.path().join("test.db").to_string_lossy().into_owned();
        let master_key = "test-master-key-for-testing-only-32b";
        let test_password = "TestP@ssw0rd!Secure";

        // Start mock servers (wiremock gives us random ports automatically)
        let mock_cloudflare = MockServer::start().await;
        let mock_ipify = MockServer::start().await;

        // Shared state to capture requests the app makes to "Cloudflare"
        let cf_requests: Arc<Mutex<Vec<ReceivedRequest>>> = Arc::new(Mutex::new(Vec::new()));

        // Mount mock handlers (see Section 5)
        mount_cloudflare_mocks(&mock_cloudflare, Arc::clone(&cf_requests)).await;
        mount_ipify_mock(&mock_ipify, "203.0.113.42").await;

        // f31: Spawn the actual app binary as a child process
        let app_process = Command::new(app_binary_path())
            .env("PORTFOLIO_PORT", app_port.to_string())
            .env("PORTFOLIO_DATA_DIR", temp_dir.path())
            .env("PORTFOLIO_MASTER_KEY", master_key)
            .env("PORTFOLIO_LOG_LEVEL", "debug")
            // KEY: Override external service URLs to point at our mocks
            .env("CLOUDFLARE_API_BASE", mock_cloudflare.uri())
            .env("IPIFY_URL", format!("{}/ip", mock_ipify.uri()))
            .env("IP_CHECK_INTERVAL", "2") // 2 seconds for fast testing
            .stdout(std::process::Stdio::piped())
            .stderr(std::process::Stdio::piped())
            .spawn()
            .map_err(|e| TestError::Setup(format!("Failed to spawn app: {e}")))?;

        let ctx = TestContext {
            app_process: Some(app_process),
            app_port,
            mock_cloudflare,
            mock_ipify,
            db_path,
            test_password: test_password.into(),
            master_key: master_key.into(),
            cf_requests,
        };

        // Ω24: Wait for app to be ready (with timeout)
        ctx.wait_for_ready(Duration::from_secs(15)).await?;

        Ok(ctx)
    }

    /// f33: Poll health endpoint until 200
    async fn wait_for_ready(&self, timeout: Duration) -> Result<(), TestError> {
        let health_url = format!("http://127.0.0.1:{}/health", self.app_port);
        let client = reqwest::Client::new();
        let start = std::time::Instant::now();

        loop {
            if start.elapsed() > timeout {
                return Err(TestError::Setup(
                    format!("App did not become ready within {}s", timeout.as_secs())
                ));
            }

            match client.get(&health_url).send().await {
                Ok(resp) if resp.status().is_success() => return Ok(()),
                _ => tokio::time::sleep(Duration::from_millis(200)).await,
            }
        }
    }

    /// f45: Cleanup — kill processes, delete temp files
    pub fn cleanup(&mut self) {
        if let Some(ref mut child) = self.app_process {
            let _ = child.kill();
            let _ = child.wait();
        }
        // MockServer drops automatically (wiremock handles this)
        // Temp dir drops automatically (tempfile handles this)
    }

    /// Helper: base URL for the running app
    pub fn base_url(&self) -> String {
        format!("http://127.0.0.1:{}", self.app_port)
    }
}

impl Drop for TestContext {
    fn drop(&mut self) {
        self.cleanup();
    }
}

/// Find the compiled app binary
fn app_binary_path() -> String {
    // In a cargo workspace, the binary is at target/debug/portfolio-server
    // or target/release/portfolio-server
    let debug_path = "target/debug/portfolio-server";
    let release_path = "target/release/portfolio-server";

    if std::path::Path::new(release_path).exists() {
        release_path.into()
    } else if std::path::Path::new(debug_path).exists() {
        debug_path.into()
    } else {
        panic!("App binary not found. Run `cargo build` first.");
    }
}

/// Ω22: Find a random free TCP port
fn find_free_port() -> u16 {
    TcpListener::bind("127.0.0.1:0")
        .unwrap()
        .local_addr()
        .unwrap()
        .port()
}
```

## 4.2 Integration Test: Complete Setup Flow

```rust
// src/integration/tests/setup_flow.rs

#[tokio::test]
async fn test_first_time_setup_creates_admin() {
    let ctx = TestContext::setup().await.unwrap();
    let client = reqwest::Client::builder()
        .redirect(reqwest::redirect::Policy::none()) // Don't follow redirects — we want to inspect them
        .build().unwrap();

    // REAL CHECK: Visiting / before setup redirects to /setup
    let resp = client.get(&format!("{}/", ctx.base_url())).send().await.unwrap();
    assert_eq!(
        resp.status(), StatusCode::SEE_OTHER,  // or 302
        "First visit before setup must redirect"
    );
    let location = resp.headers().get("location").unwrap().to_str().unwrap();
    assert!(
        location.contains("/setup"),
        "Must redirect to /setup, got: {location}"
    );

    // REAL CHECK: POST /setup creates admin account
    let resp = client.post(&format!("{}/setup", ctx.base_url()))
        .json(&serde_json::json!({
            "username": "admin",
            "password": &ctx.test_password
        }))
        .send().await.unwrap();

    assert!(
        resp.status().is_success() || resp.status() == StatusCode::SEE_OTHER,
        "Setup POST must succeed, got: {}",
        resp.status()
    );

    // REAL CHECK: Setup endpoint is now disabled (can't create second admin)
    let resp = client.post(&format!("{}/setup", ctx.base_url()))
        .json(&serde_json::json!({
            "username": "hacker",
            "password": "trying_to_create_second_admin"
        }))
        .send().await.unwrap();

    assert!(
        resp.status().is_client_error() || resp.status().is_server_error(),
        "Second setup attempt must be rejected, got: {}",
        resp.status()
    );

    // REAL CHECK: Can now log in with created credentials
    let resp = client.post(&format!("{}/login", ctx.base_url()))
        .json(&serde_json::json!({
            "username": "admin",
            "password": &ctx.test_password
        }))
        .send().await.unwrap();

    assert!(
        resp.status().is_success() || resp.status() == StatusCode::SEE_OTHER,
        "Login with new credentials must succeed, got: {}",
        resp.status()
    );
}
```

## 4.3 Integration Test: DNS Update Cycle

```rust
// src/integration/tests/dns_cycle.rs

#[tokio::test]
async fn test_dns_monitor_detects_ip_change_and_updates_cloudflare() {
    let ctx = TestContext::setup().await.unwrap();
    let client = reqwest::Client::new();

    // First: complete setup and save Cloudflare settings
    setup_admin_and_login(&ctx, &client).await;
    save_cloudflare_settings(&ctx, &client, "fake_cf_token", "zone123", "record456").await;

    // The mock ipify is returning "203.0.113.42"
    // The app's DNS monitor should detect this and call mock Cloudflare

    // Wait for the DNS monitor to run (interval set to 2s in test config)
    tokio::time::sleep(Duration::from_secs(5)).await;

    // REAL CHECK: The app actually called our mock Cloudflare API
    let requests = ctx.cf_requests.lock().await;
    assert!(
        !requests.is_empty(),
        "App must have made at least one request to Cloudflare API. Got 0 requests."
    );

    // REAL CHECK: The request was a PATCH to the correct endpoint
    let dns_update = requests.iter()
        .find(|r| r.method == "PATCH" && r.path.contains("dns_records"))
        .expect("Must find a PATCH request to dns_records endpoint");

    assert!(
        dns_update.path.contains("zone123"),
        "Request must target correct zone. Path: {}",
        dns_update.path
    );
    assert!(
        dns_update.path.contains("record456"),
        "Request must target correct record. Path: {}",
        dns_update.path
    );

    // REAL CHECK: The request body contains the detected IP
    let body: serde_json::Value = serde_json::from_str(&dns_update.body).unwrap();
    assert_eq!(
        body["content"], "203.0.113.42",
        "DNS update must contain the IP from ipify"
    );
    assert_eq!(
        body["type"], "A",
        "Must be an A record update"
    );

    // REAL CHECK: Authorization header was sent
    let auth_header = dns_update.headers.iter()
        .find(|(k, _)| k.to_lowercase() == "authorization")
        .expect("Must include Authorization header");
    assert!(
        auth_header.1.contains("Bearer"),
        "Auth header must be Bearer token format"
    );

    // Now simulate IP change: update mock ipify to return different IP
    mount_ipify_mock(&ctx.mock_ipify, "198.51.100.99").await;

    // Clear request log
    ctx.cf_requests.lock().await.clear();

    // Wait for next check cycle
    tokio::time::sleep(Duration::from_secs(5)).await;

    // REAL CHECK: App detected the change and sent another update
    let requests = ctx.cf_requests.lock().await;
    let update = requests.iter()
        .find(|r| r.method == "PATCH")
        .expect("Must send update after IP change");

    let body: serde_json::Value = serde_json::from_str(&update.body).unwrap();
    assert_eq!(
        body["content"], "198.51.100.99",
        "DNS update must contain the NEW IP"
    );
}

#[tokio::test]
async fn test_dns_monitor_does_not_update_when_ip_unchanged() {
    let ctx = TestContext::setup().await.unwrap();
    let client = reqwest::Client::new();

    setup_admin_and_login(&ctx, &client).await;
    save_cloudflare_settings(&ctx, &client, "fake_cf_token", "zone123", "record456").await;

    // Wait for first update
    tokio::time::sleep(Duration::from_secs(5)).await;

    // Record how many requests were made
    let initial_count = ctx.cf_requests.lock().await.len();

    // Wait for another cycle — IP hasn't changed
    tokio::time::sleep(Duration::from_secs(5)).await;

    let final_count = ctx.cf_requests.lock().await.len();

    // REAL CHECK: No additional PATCH requests when IP is unchanged
    let new_patches = ctx.cf_requests.lock().await[initial_count..]
        .iter()
        .filter(|r| r.method == "PATCH")
        .count();

    assert_eq!(
        new_patches, 0,
        "Must NOT send DNS update when IP is unchanged. Got {new_patches} extra PATCH requests."
    );
}
```

## 4.4 Integration Test: Token Encryption at Rest

```rust
// src/integration/tests/encryption.rs

#[tokio::test]
async fn test_cloudflare_token_is_encrypted_in_database() {
    let ctx = TestContext::setup().await.unwrap();
    let client = reqwest::Client::new();

    setup_admin_and_login(&ctx, &client).await;

    let plaintext_token = "cf_super_secret_api_token_12345";
    save_cloudflare_settings(&ctx, &client, plaintext_token, "zone1", "rec1").await;

    // f47: REAL CHECK — Read the raw database and verify the token is NOT stored in plaintext
    // This is the chicken-and-egg bypass: use shell injection to read the DB directly
    let result = inject_shell(&format!(
        "sqlite3 {} &quot;SELECT hex(value) FROM settings WHERE key='cf_token'&quot;",
        ctx.db_path
    )).unwrap();

    let hex_value = result.stdout.trim();

    // The hex representation of the stored value must NOT contain
    // the hex representation of the plaintext token
    let plaintext_hex = hex::encode(plaintext_token);
    assert!(
        !hex_value.contains(&plaintext_hex.to_uppercase()),
        "Plaintext token found in database! Encryption is broken.\n\
         Stored (hex): {hex_value}\n\
         Plaintext (hex): {plaintext_hex}"
    );

    // Also check: the raw bytes must not contain the ASCII plaintext
    let result = inject_shell(&format!(
        "sqlite3 {} &quot;SELECT value FROM settings WHERE key='cf_token'&quot; | strings",
        ctx.db_path
    )).unwrap();

    assert!(
        !result.stdout.contains(plaintext_token),
        "Plaintext token found via strings command! Encryption is broken."
    );

    // REAL CHECK: The stored value is at least as long as nonce + ciphertext + auth tag
    // AES-256-GCM: 12 byte nonce + len(plaintext) + 16 byte tag
    let expected_min_len = 12 + plaintext_token.len() + 16;
    let stored_len = hex_value.len() / 2; // hex is 2 chars per byte
    assert!(
        stored_len >= expected_min_len,
        "Stored value too short for AES-GCM. Expected >= {expected_min_len} bytes, got {stored_len}"
    );
}
```

## 4.5 Integration Test: Security Boundaries

```rust
// src/integration/tests/security.rs

#[tokio::test]
async fn test_admin_routes_require_authentication() {
    let ctx = TestContext::setup().await.unwrap();
    let client = reqwest::Client::builder()
        .redirect(reqwest::redirect::Policy::none())
        .build().unwrap();

    // Complete setup first
    setup_admin(&ctx, &client).await;

    // REAL CHECK: Every admin route rejects unauthenticated requests
    let protected_routes = vec![
        ("GET", "/admin"),
        ("GET", "/admin/settings"),
        ("POST", "/admin/settings"),
    ];

    for (method, path) in &protected_routes {
        let resp = match *method {
            "GET" => client.get(&format!("{}{}", ctx.base_url(), path)).send().await.unwrap(),
            "POST" => client.post(&format!("{}{}", ctx.base_url(), path))
                .json(&serde_json::json!({"test": "data"}))
                .send().await.unwrap(),
            _ => unreachable!(),
        };

        let status = resp.status();
        assert!(
            status == StatusCode::UNAUTHORIZED
            || status == StatusCode::SEE_OTHER
            || status == StatusCode::TEMPORARY_REDIRECT,
            "{method} {path} must reject unauthenticated request. Got: {status}"
        );
    }
}

#[tokio::test]
async fn test_expired_session_is_rejected() {
    let ctx = TestContext::setup().await.unwrap();
    let client = reqwest::Client::builder()
        .redirect(reqwest::redirect::Policy::none())
        .build().unwrap();

    setup_admin(&ctx, &client).await;
    let session_cookie = login_and_get_cookie(&ctx, &client).await;

    // Manually expire the session in the database using shell injection
    inject_shell(&format!(
        "sqlite3 {} &quot;UPDATE sessions SET expires_at = datetime('now', '-1 hour')&quot;",
        ctx.db_path
    )).unwrap();

    // REAL CHECK: Request with expired session is rejected
    let resp = client.get(&format!("{}/admin", ctx.base_url()))
        .header("Cookie", format!("session_id={session_cookie}"))
        .send().await.unwrap();

    assert!(
        resp.status() == StatusCode::UNAUTHORIZED || resp.status() == StatusCode::SEE_OTHER,
        "Expired session must be rejected. Got: {}",
        resp.status()
    );
}

#[tokio::test]
async fn test_error_responses_dont_leak_internals() {
    let ctx = TestContext::setup().await.unwrap();
    let client = reqwest::Client::new();

    // Hit a route that will cause an internal error
    // (e.g., try to save settings before setup — should error, not crash)
    let resp = client.post(&format!("{}/admin/settings", ctx.base_url()))
        .json(&serde_json::json!({"api_token": "test"}))
        .send().await.unwrap();

    let body = resp.text().await.unwrap();

    // REAL CHECK: Response body does not contain internal details
    let forbidden_patterns = vec![
        "sqlite", "SQLITE", "database", "panic", "unwrap",
        "thread", "stack", "backtrace", "/src/", ".rs:",
        "sqlx", "tokio", "hyper",
    ];

    for pattern in &forbidden_patterns {
        assert!(
            !body.to_lowercase().contains(&pattern.to_lowercase()),
            "Error response contains internal detail '{pattern}'.\nFull body: {body}"
        );
    }
}
```

---

# 5. MOCK API IMPLEMENTATION

## 5.1 Mock Cloudflare API (Realistic)

```rust
// src/mocks/cloudflare.rs

use wiremock::{MockServer, Mock, ResponseTemplate};
use wiremock::matchers::{method, path, path_regex, header};

/// Mount all Cloudflare API mock handlers
pub async fn mount_cloudflare_mocks(
    server: &MockServer,
    request_log: Arc<Mutex<Vec<ReceivedRequest>>>,
) {
    // Token verification endpoint
    Mock::given(method("GET"))
        .and(path("/client/v4/user/tokens/verify"))
        .and(header("Authorization", "Bearer fake_cf_token"))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
            "result": {
                "id": "test_token_id",
                "status": "active"
            },
            "success": true,
            "errors": [],
            "messages": [{"code": 10000, "message": "This API Token is valid and active"}]
        })))
        .mount(server)
        .await;

    // Token verification — invalid token
    Mock::given(method("GET"))
        .and(path("/client/v4/user/tokens/verify"))
        .respond_with(ResponseTemplate::new(401).set_body_json(serde_json::json!({
            "success": false,
            "errors": [{"code": 1000, "message": "Invalid API Token"}],
            "messages": [],
            "result": null
        })))
        .mount(server)
        .await;

    // DNS record update (PATCH) — capture and log the request
    let log = Arc::clone(&request_log);
    Mock::given(method("PATCH"))
        .and(path_regex(r"/client/v4/zones/.+/dns_records/.+"))
        .respond_with(move |req: &wiremock::Request| {
            // Capture the request for assertion
            let received = ReceivedRequest {
                method: req.method.to_string(),
                path: req.url.path().to_string(),
                body: String::from_utf8_lossy(&req.body).into_owned(),
                headers: req.headers.iter()
                    .map(|(k, v)| (k.to_string(), v.to_str().unwrap_or("").to_string()))
                    .collect(),
                timestamp: chrono::Utc::now(),
            };

            // Use a blocking lock since wiremock handlers are sync
            let mut log = log.blocking_lock();
            log.push(received);

            ResponseTemplate::new(200).set_body_json(serde_json::json!({
                "result": {
                    "id": "record456",
                    "zone_id": "zone123",
                    "zone_name": "example.com",
                    "name": "home.example.com",
                    "type": "A",
                    "content": "203.0.113.42",
                    "proxiable": true,
                    "proxied": false,
                    "ttl": 300,
                    "locked": false,
                    "meta": {
                        "auto_added": false,
                        "managed_by_apps": false,
                        "managed_by_argo_tunnel": false
                    },
                    "comment": null,
                    "tags": [],
                    "created_on": "2024-01-01T00:00:00.000000Z",
                    "modified_on": "2024-06-15T12:00:00.000000Z"
                },
                "success": true,
                "errors": [],
                "messages": []
            }))
        })
        .mount(server)
        .await;

    // List DNS records (GET) — for initial record discovery
    Mock::given(method("GET"))
        .and(path_regex(r"/client/v4/zones/.+/dns_records"))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
            "result": [
                {
                    "id": "record456",
                    "zone_id": "zone123",
                    "name": "home.example.com",
                    "type": "A",
                    "content": "192.0.2.1",
                    "ttl": 300,
                    "proxied": false
                },
                {
                    "id": "record789",
                    "zone_id": "zone123",
                    "name": "home.example.com",
                    "type": "AAAA",
                    "content": "2001:db8::1",
                    "ttl": 300,
                    "proxied": false
                }
            ],
            "success": true,
            "errors": [],
            "messages": [],
            "result_info": {
                "page": 1,
                "per_page": 100,
                "count": 2,
                "total_count": 2,
                "total_pages": 1
            }
        })))
        .mount(server)
        .await;

    // Cloudflare API rate limit simulation (429)
    // This mock has lower priority — only triggers if we explicitly test rate limiting
    Mock::given(method("PATCH"))
        .and(path_regex(r"/client/v4/zones/.+/dns_records/.+"))
        .and(header("X-Test-Rate-Limit", "true"))
        .respond_with(ResponseTemplate::new(429).set_body_json(serde_json::json!({
            "success": false,
            "errors": [{"code": 10000, "message": "Rate limit exceeded"}],
            "messages": []
        })))
        .mount(server)
        .await;
}
```

## 5.2 Mock IP Detection Service

```rust
// src/mocks/ipify.rs

use wiremock::{MockServer, Mock, ResponseTemplate};
use wiremock::matchers::{method, path};

/// Mount ipify mock that returns a configurable IP address
pub async fn mount_ipify_mock(server: &MockServer, ip: &str) {
    // Reset existing mocks to allow IP changes during tests
    server.reset().await;

    // Plain text IP response (matches real ipify behavior)
    Mock::given(method("GET"))
        .and(path("/ip"))
        .respond_with(
            ResponseTemplate::new(200)
                .set_body_string(ip.to_string())
        )
        .mount(server)
        .await;

    // JSON format (alternative endpoint)
    Mock::given(method("GET"))
        .and(path("/"))
        .respond_with(
            ResponseTemplate::new(200)
                .set_body_json(serde_json::json!({
                    "ip": ip
                }))
        )
        .mount(server)
        .await;
}

/// Mount a failing ipify mock (simulates network issues)
pub async fn mount_ipify_failure(server: &MockServer) {
    server.reset().await;

    Mock::given(method("GET"))
        .respond_with(ResponseTemplate::new(503).set_body_string("Service Unavailable"))
        .mount(server)
        .await;
}
```

## 5.3 How Traffic Routing Works

The key to the mock infrastructure is **environment variable injection**. The app binary must support configurable base URLs for external services:

```rust
// In the app's config.rs (m5):

#[derive(Clone, Debug)]
pub struct Config {
    // ... existing fields ...

    /// Base URL for Cloudflare API (default: https://api.cloudflare.com)
    /// Override in tests to point at mock server
    pub cloudflare_api_base: String,

    /// URL for IP detection (default: https://api.ipify.org)
    /// Override in tests to point at mock server
    pub ipify_url: String,
}

impl Config {
    pub fn load() -> Result<Self, ConfigError> {
        Ok(Config {
            // ...
            cloudflare_api_base: std::env::var("CLOUDFLARE_API_BASE")
                .unwrap_or_else(|_| "https://api.cloudflare.com".into()),
            ipify_url: std::env::var("IPIFY_URL")
                .unwrap_or_else(|_| "https://api.ipify.org".into()),
        })
    }
}

// In dns/cloudflare.rs — use config.cloudflare_api_base instead of hardcoded URL:
pub async fn update_dns(config: &Config, /* ... */) -> Result<(), AppError> {
    let url = format!(
        "{}/client/v4/zones/{}/dns_records/{}",
        config.cloudflare_api_base, zone_id, record_id
    );
    // ... rest of implementation
}
```

**Traffic flow in tests:**
```
App binary (child process)
  │
  │  env: CLOUDFLARE_API_BASE=http://127.0.0.1:54321
  │  env: IPIFY_URL=http://127.0.0.1:54322/ip
  │
  ├──► HTTP GET http://127.0.0.1:54322/ip
  │    └──► wiremock mock_ipify → returns "203.0.113.42"
  │
  ├──► HTTP PATCH http://127.0.0.1:54321/client/v4/zones/zone123/dns_records/record456
  │    └──► wiremock mock_cloudflare → logs request, returns success JSON
  │
  └──► Test binary reads mock_cloudflare request log → asserts correctness
```

No monkey-patching. No trait object injection. No compile-time feature flags for test vs production HTTP clients. Just environment variables that swap URLs. The app code is identical in test and production — only the URLs differ.

---

# 6. CHICKEN-AND-EGG BOOTSTRAP PATTERNS

## 6.1 The Problem

Some things can't be tested from inside the system being tested:
- "Is the token actually encrypted on disk?" — the app thinks it is, but you need to read the raw bytes to be sure
- "Does the page actually render correctly in a browser?" — the server sends HTML, but does it look right?
- "Is the binary actually small?" — you can't measure your own binary size from inside
- "Does the systemd service file work?" — you need an OS-level check

## 6.2 Shell Injection for System-Level Verification

```rust
// src/bootstrap/system_checks.rs

/// Verify the production binary size is within acceptable limits
pub fn check_binary_size() -> Result<(), TestError> {
    let result = inject_shell(
        "ls -la target/release/portfolio-server | awk '{print $5}'"
    )?;

    let size_bytes: u64 = result.stdout.trim().parse()
        .map_err(|_| TestError::Bootstrap("Could not parse binary size".into()))?;

    let size_mb = size_bytes as f64 / 1_048_576.0;

    assert!(
        size_mb < 20.0,
        "Release binary is {size_mb:.1}MB — exceeds 20MB limit. Check for bloat."
    );

    assert!(
        size_mb > 1.0,
        "Release binary is only {size_mb:.1}MB — suspiciously small. Assets may not be embedded."
    );

    println!("Binary size: {size_mb:.1}MB ✓");
    Ok(())
}

/// Verify the binary has no dynamic dependencies we don't expect
pub fn check_dynamic_deps() -> Result<(), TestError> {
    let result = inject_shell("ldd target/release/portfolio-server 2>&1 || true")?;

    let output = result.stdout.to_lowercase();

    // These are acceptable
    let allowed = vec!["libc", "libm", "libpthread", "libdl", "librt", "libgcc", "ld-linux", "linux-vdso"];

    for line in output.lines() {
        if line.contains("not found") {
            return Err(TestError::Bootstrap(format!("Missing dependency: {line}")));
        }

        // Check each linked library is in our allowed list
        if line.contains("=>") {
            let lib_name = line.split_whitespace().next().unwrap_or("");
            let is_allowed = allowed.iter().any(|a| lib_name.contains(a));
            if !is_allowed && !lib_name.is_empty() {
                println!("WARNING: Unexpected dynamic dependency: {lib_name}");
            }
        }
    }

    Ok(())
}

/// Verify SQLite database file permissions after app creates it
pub fn check_db_permissions(db_path: &str) -> Result<(), TestError> {
    let result = inject_shell(&format!("stat -c '%a' {db_path}"))?;
    let perms = result.stdout.trim();

    assert!(
        perms == "600" || perms == "640" || perms == "644",
        "Database file permissions are {perms} — should be 600 or 640 for security"
    );

    Ok(())
}

/// Verify the app doesn't leave orphan processes
pub fn check_no_orphan_processes(app_port: u16) -> Result<(), TestError> {
    let result = inject_shell(&format!(
        "lsof -i :{app_port} -t 2>/dev/null | wc -l"
    ))?;

    let count: i32 = result.stdout.trim().parse().unwrap_or(0);
    assert_eq!(
        count, 0,
        "Found {count} processes still listening on port {app_port} after cleanup"
    );

    Ok(())
}
```

## 6.3 Python Injection for Complex Verification

```rust
// src/bootstrap/complex_checks.rs

/// Verify that all pages return valid HTML (not just 200 OK)
pub fn check_html_validity(base_url: &str) -> Result<(), TestError> {
    let script = format!(r#"
import json
from html.parser import HTMLParser

class HTMLValidator(HTMLParser):
    def __init__(self):
        super().__init__()
        self.errors = []
        self.tag_stack = []

    def handle_starttag(self, tag, attrs):
        void_elements = {{'area','base','br','col','embed','hr','img','input','link','meta','source','track','wbr'}}
        if tag not in void_elements:
            self.tag_stack.append(tag)

    def handle_endtag(self, tag):
        if self.tag_stack and self.tag_stack[-1] == tag:
            self.tag_stack.pop()
        elif tag in self.tag_stack:
            self.errors.append(f"Misnested tag: {{tag}}")

import urllib.request

pages = ["/", "/resume", "/whitepaper", "/innovation", "/login"]
results = []

for page in pages:
    try:
        resp = urllib.request.urlopen(f"{base_url}{{page}}")
        html = resp.read().decode()

        validator = HTMLValidator()
        validator.feed(html)

        results.append({{
            "page": page,
            "status": resp.status,
            "has_doctype": html.strip().lower().startswith("<!doctype"),
            "has_html_tag": "<html" in html.lower(),
            "has_head": "<head" in html.lower(),
            "has_body": "<body" in html.lower(),
            "parse_errors": validator.errors[:5],
            "valid": resp.status == 200 and html.strip().lower().startswith("<!doctype")
        }})
    except Exception as e:
        results.append({{
            "page": page,
            "status": 0,
            "valid": False,
            "error": str(e)
        }})

all_valid = all(r["valid"] for r in results)
print(json.dumps([{{
    "passed": all_valid,
    "test_name": "html_validity",
    "duration_ms": 0,
    "screenshot_path": None,
    "error": None if all_valid else "Invalid HTML detected",
    "assertions": [
        {{
            "description": f"{{r['page']}} returns valid HTML",
            "passed": r["valid"],
            "expected": "Valid HTML with doctype",
            "actual": f"Status {{r.get('status', 'N/A')}}, doctype={{r.get('has_doctype', False)}}"
        }}
        for r in results
    ]
}}]))
"#);

    let results = inject_python(&script)?;
    for result in &results {
        if !result.passed {
            return Err(TestError::Validation(format!(
                "HTML validity check failed: {:?}",
                result.assertions.iter().filter(|a| !a.passed).collect::<Vec<_>>()
            )));
        }
    }
    Ok(())
}

/// Verify response headers include security headers
pub fn check_security_headers(base_url: &str) -> Result<(), TestError> {
    let script = format!(r#"
import json, urllib.request

resp = urllib.request.urlopen("{base_url}/")
headers = dict(resp.headers)

checks = []

# Must have X-Content-Type-Options
checks.append({{
    "header": "X-Content-Type-Options",
    "present": "X-Content-Type-Options" in headers,
    "value": headers.get("X-Content-Type-Options", "MISSING"),
    "expected": "nosniff"
}})

# Must have X-Frame-Options
checks.append({{
    "header": "X-Frame-Options",
    "present": "X-Frame-Options" in headers,
    "value": headers.get("X-Frame-Options", "MISSING"),
    "expected": "DENY or SAMEORIGIN"
}})

# Should have Content-Security-Policy
checks.append({{
    "header": "Content-Security-Policy",
    "present": "Content-Security-Policy" in headers,
    "value": headers.get("Content-Security-Policy", "MISSING")[:100],
    "expected": "Present"
}})

all_present = all(c["present"] for c in checks)
print(json.dumps([{{
    "passed": all_present,
    "test_name": "security_headers",
    "duration_ms": 0,
    "screenshot_path": None,
    "error": None if all_present else "Missing security headers",
    "assertions": [
        {{
            "description": f"{{c['header']}} header present",
            "passed": c["present"],
            "expected": c["expected"],
            "actual": c["value"]
        }}
        for c in checks
    ]
}}]))
"#);

    let results = inject_python(&script)?;
    for result in &results {
        assert!(result.passed, "Security headers check failed: {:?}", result.error);
    }
    Ok(())
}
```

---

# 7. DIRECTORY STRUCTURE AND CRATE RECOMMENDATIONS

## 7.1 Workspace Layout

```
portfolio-workspace/
├── Cargo.toml                          # [workspace] definition
│
├── crates/
│   ├── core/                           # Shared library — business logic
│   │   ├── Cargo.toml
│   │   └── src/
│   │       ├── lib.rs
│   │       ├── config.rs               # m5
│   │       ├── error.rs                # m7
│   │       ├── crypto/
│   │       │   ├── mod.rs
│   │       │   └── token.rs            # f14, f15 (encrypt/decrypt)
│   │       ├── auth/
│   │       │   ├── mod.rs
│   │       │   ├── password.rs         # f16, f17 (hash/verify)
│   │       │   └── session.rs          # Session struct, validation logic
│   │       ├── dns/
│   │       │   ├── mod.rs
│   │       │   ├── cloudflare.rs       # f11, API client
│   │       │   └── ip_detect.rs        # f13
│   │       └── db/
│   │           ├── mod.rs
│   │           └── queries.rs
│   │
│   ├── app/                            # Binary 1: production server
│   │   ├── Cargo.toml                  # depends on `core`
│   │   └── src/
│   │       ├── main.rs                 # f0
│   │       ├── web/
│   │       │   ├── mod.rs              # f1 (router)
│   │       │   ├── pages.rs            # f2-f5
│   │       │   ├── admin.rs            # f6-f9
│   │       │   ├── setup.rs            # f28
│   │       │   └── middleware.rs       # f24, f25
│   │       ├── dns/
│   │       │   └── monitor.rs          # f12 (background loop)
│   │       ├── templates/              # Askama .html files
│   │       └── assets/                 # rust-embed static files
│   │
│   └── test-runner/                    # Binary 2: self-evaluating tests
│       ├── Cargo.toml                  # depends on `core` + test deps
│       └── src/
│           ├── main.rs                 # f30: test orchestrator
│           ├── context.rs              # t20: TestContext
│           ├── injection/
│           │   ├── mod.rs              # Ω20, Ω21 (Python/shell injection)
│           │   └── scripts.rs          # Embedded Python test scripts
│           ├── mocks/
│           │   ├── mod.rs
│           │   ├── cloudflare.rs       # Mock CF API (wiremock)
│           │   └── ipify.rs            # Mock IP service
│           ├── unit_tests/
│           │   ├── mod.rs              # f49
│           │   ├── crypto_tests.rs
│           │   ├── password_tests.rs
│           │   ├── dns_tests.rs
│           │   ├── session_tests.rs
│           │   └── error_tests.rs
│           ├── integration_tests/
│           │   ├── mod.rs              # f50
│           │   ├── setup_flow.rs
│           │   ├── dns_cycle.rs
│           │   ├── encryption.rs
│           │   └── security.rs
│           ├── ui_tests/
│           │   ├── mod.rs              # f51
│           │   ├── runner.rs           # UI test orchestration
│           │   ├── scripts.rs          # Playwright Python scripts
│           │   └── visual.rs           # Screenshot comparison
│           ├── bootstrap/
│           │   ├── mod.rs
│           │   ├── system_checks.rs    # Binary size, deps, permissions
│           │   └── complex_checks.rs   # HTML validity, security headers
│           └── report.rs               # f52: test report generation
│
├── test_baselines/                     # Committed screenshot baselines
│   ├── index.png
│   ├── resume.png
│   ├── whitepaper.png
│   ├── innovation.png
│   └── login.png
│
└── migrations/                         # Shared SQLite migrations
    ├── 001_initial.sql
    └── 002_dns_log.sql
```

## 7.2 Workspace Cargo.toml

```toml
[workspace]
members = [
    "crates/core",
    "crates/app",
    "crates/test-runner",
]
resolver = "2"

[workspace.dependencies]
# Shared across workspace
serde = { version = "1", features = ["derive"] }
serde_json = "1"
tokio = { version = "1", features = ["full"] }
sqlx = { version = "0.8", features = ["runtime-tokio", "sqlite", "migrate"] }
reqwest = { version = "0.12", features = ["json", "rustls-tls"], default-features = false }
tracing = "0.1"
anyhow = "1"
thiserror = "2"
chrono = { version = "0.4", features = ["serde"] }
uuid = { version = "1", features = ["v4", "serde"] }
```

## 7.3 Test Runner Cargo.toml

```toml
[package]
name = "test-runner"
version = "0.1.0"
edition = "2021"

[[bin]]
name = "test-runner"
path = "src/main.rs"

[dependencies]
# Shared business logic
core = { path = "../core" }

# Workspace deps
serde.workspace = true
serde_json.workspace = true
tokio.workspace = true
reqwest.workspace = true
tracing.workspace = true
anyhow.workspace = true
chrono.workspace = true
uuid.workspace = true

# Test-specific deps
wiremock = "0.6"              # Mock HTTP servers
tempfile = "3"                # Temporary directories for test DBs
hex = "0.4"                   # Hex encoding for DB inspection
colored = "2"                 # Colored terminal output for reports
tracing-subscriber = "0.3"

# NOT included: Playwright, Pillow — these are Python deps installed at runtime via Ω20
```

## 7.4 Crate Recommendations Summary

| Purpose | Crate | Used In | Why |
|---------|-------|---------|-----|
| Mock HTTP server | `wiremock` | test-runner | Best Rust mock server — request matching, recording, async |
| Temp files/dirs | `tempfile` | test-runner | Isolated test databases, auto-cleanup on drop |
| Hex encoding | `hex` | test-runner | Inspect raw DB bytes for encryption verification |
| Colored output | `colored` | test-runner | Readable test reports in terminal |
| Browser automation | Playwright (Python) | injected | Gold standard — no Rust equivalent at this quality |
| Image comparison | Pillow (Python) | injected | Pixel-level screenshot diff |
| HTML validation | html.parser (Python stdlib) | injected | Structural HTML validation |

---

# 8. TEST REPORT GENERATION

## 8.1 Test Runner Entry Point

```rust
// crates/test-runner/src/main.rs

use std::time::Instant;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt::init();

    let start = Instant::now();
    let mut report = TestReport::new();

    // ═══════════════════════════════════════════
    // PHASE 1: UNIT TESTS (no processes needed)
    // ═══════════════════════════════════════════
    println!("\n{}", "═".repeat(60));
    println!("  PHASE 1: UNIT TESTS");
    println!("{}\n", "═".repeat(60));

    let unit_results = run_unit_suite();
    report.add_phase("Unit Tests", unit_results);

    // ═══════════════════════════════════════════
    // PHASE 2: BOOTSTRAP CHECKS
    // ═══════════════════════════════════════════
    println!("\n{}", "═".repeat(60));
    println!("  PHASE 2: BOOTSTRAP CHECKS");
    println!("{}\n", "═".repeat(60));

    report.add_check("Binary size", check_binary_size());
    report.add_check("Dynamic dependencies", check_dynamic_deps());

    // ═══════════════════════════════════════════
    // PHASE 3: INTEGRATION TESTS (spawn app + mocks)
    // ═══════════════════════════════════════════
    println!("\n{}", "═".repeat(60));
    println!("  PHASE 3: INTEGRATION TESTS");
    println!("{}\n", "═".repeat(60));

    let mut ctx = TestContext::setup().await?;
    let integration_results = run_integration_suite(&ctx).await;
    report.add_phase("Integration Tests", integration_results);

    // ═══════════════════════════════════════════
    // PHASE 4: UI TESTS (inject Python/Playwright)
    // ═══════════════════════════════════════════
    println!("\n{}", "═".repeat(60));
    println!("  PHASE 4: UI TESTS");
    println!("{}\n", "═".repeat(60));

    let ui_results = run_ui_suite(&ctx).await;
    report.add_phase("UI Tests", ui_results);

    // ═══════════════════════════════════════════
    // PHASE 5: COMPLEX VERIFICATION (Python injection)
    // ═══════════════════════════════════════════
    println!("\n{}", "═".repeat(60));
    println!("  PHASE 5: COMPLEX VERIFICATION");
    println!("{}\n", "═".repeat(60));

    report.add_check("HTML validity", check_html_validity(&ctx.base_url()));
    report.add_check("Security headers", check_security_headers(&ctx.base_url()));

    // Cleanup
    ctx.cleanup();
    report.add_check("No orphan processes", check_no_orphan_processes(ctx.app_port));

    // ═══════════════════════════════════════════
    // REPORT
    // ═══════════════════════════════════════════
    report.total_duration = start.elapsed();
    report.print_summary();
    report.save_json("test_report.json")?;

    if report.has_failures() {
        std::process::exit(1);
    }

    Ok(())
}
```

## 8.2 Report Output Format

```
══════════════════════════════════════════════════════════════
  TEST REPORT — portfolio-server
══════════════════════════════════════════════════════════════

  PHASE 1: Unit Tests                          12/12 PASSED ✓
    ✓ encrypt_decrypt_roundtrip                     1ms
    ✓ encrypted_output_is_not_plaintext             0ms
    ✓ different_inputs_different_outputs             0ms
    ✓ same_input_different_ciphertexts              1ms
    ✓ wrong_key_fails_decryption                    0ms
    ✓ tampered_ciphertext_rejected                  0ms
    ✓ hash_is_not_plaintext                        89ms
    ✓ correct_password_verifies                    87ms
    ✓ wrong_password_fails                         88ms
    ✓ same_password_different_hashes              175ms
    ✓ hashing_minimum_time                         91ms
    ✓ session_id_valid_uuid                         0ms

  PHASE 2: Bootstrap Checks                     2/2 PASSED ✓
    ✓ Binary size: 12.3MB
    ✓ Dynamic dependencies: OK

  PHASE 3: Integration Tests                    5/5 PASSED ✓
    ✓ first_time_setup_creates_admin              312ms
    ✓ dns_monitor_detects_ip_change              5204ms
    ✓ dns_no_update_when_unchanged               5118ms
    ✓ token_encrypted_in_database                 287ms
    ✓ admin_routes_require_auth                   156ms

  PHASE 4: UI Tests                             3/3 PASSED ✓
    ✓ resume_page_content_and_structure           892ms  📸
    ✓ admin_login_flow                           1204ms  📸
    ✓ admin_login_rejection                       743ms

  PHASE 5: Complex Verification                 2/2 PASSED ✓
    ✓ HTML validity: all 5 pages valid
    ✓ Security headers: all present

══════════════════════════════════════════════════════════════
  TOTAL: 24/24 PASSED    Duration: 14.2s    EXIT: 0
══════════════════════════════════════════════════════════════
```

---

# APPENDIX: KEY DESIGN DECISIONS

## Why wiremock over mockito or httpmock?

`wiremock` is async-native (built on Tokio), supports request recording (critical for our assertion pattern where we check what the app sent to "Cloudflare"), and allows dynamic response generation via closures. `mockito` is sync-only. `httpmock` is good but wiremock's API is cleaner for our use case.

## Why Python injection over pure Rust browser automation?

Playwright (Python) supports: full browser lifecycle, network interception, cookie inspection, JavaScript error capture, screenshot comparison, mobile viewport emulation, and accessibility testing. The best Rust alternative (`thirtyfour`) requires a separate WebDriver binary and supports maybe 40% of these features. The 200ms overhead of spawning a Python process is irrelevant in a test suite that takes 14 seconds total.

## Why a separate binary instead of `cargo test`?

`cargo test` runs tests in the library's process. Integration tests need to spawn the actual compiled binary as a child process to test real behavior (port binding, signal handling, file creation, environment variable parsing). The test-runner binary is the orchestrator that treats the app binary as a black box — exactly how a real user or deployment system would interact with it.

## Why shell injection for DB inspection?

The app encrypts tokens before writing to SQLite. If we test encryption by calling `encrypt()` then `decrypt()` in the same process, we're testing our own code with our own code — an ice cream cone. By using `sqlite3` CLI to read the raw bytes, we verify that what's actually on disk is encrypted, independent of our Rust code's opinion about it.