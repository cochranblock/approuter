//! f49–f60 self-eval. Only compiled with tests feature. whyyoulying-test binary runs this.

use std::path::PathBuf;
use std::process::Command;

/// Path to release binary (whyyoulying). E2E tests invoke it.
fn release_bin() -> PathBuf {
    let exe = std::env::current_exe().expect("current exe");
    let dir = exe.parent().expect("parent");
    let stem = exe.file_stem().and_then(|s| s.to_str()).unwrap_or("whyyoulying-test");
    let release_stem = stem.strip_suffix("-test").unwrap_or("whyyoulying");
    let suffix = std::env::consts::EXE_SUFFIX;
    dir.join(format!("{release_stem}{suffix}"))
}

fn fixtures_path() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("fixtures")
}

fn run_bin(args: &[&str]) -> (std::process::Output, String, String) {
    let exe = release_bin();
    let out = Command::new(&exe).args(args).output().unwrap();
    let stdout = String::from_utf8(out.stdout.clone()).unwrap();
    let stderr = String::from_utf8(out.stderr.clone()).unwrap();
    (out, stdout, stderr)
}

/// f30 = run_tests
pub fn f30() -> i32 {
    let mut failed = 0;
    let green = "\x1b[32m";
    let red = "\x1b[31m";
    let reset = "\x1b[0m";

    for (name, pass) in [
        ("f49 unit", f49()),
        ("f50 integration", f50()),
        ("f51 e2e", f51()),
        ("f52 run fixtures", f52()),
        ("f53 min-confidence", f53()),
        ("f54 ingest", f54()),
        ("f55 missing data_path", f55()),
        ("f56 agency filter", f56()),
        ("f57 csv output", f57()),
        ("f58 export-referral", f58()),
        ("f59 export-fbi", f59()),
        ("f60 empty exit zero", f60()),
    ] {
        print!("{name}: ");
        if pass {
            println!("{green}PASS{reset}");
        } else {
            println!("{red}FAIL{reset}");
            failed += 1;
        }
    }

    if failed > 0 {
        eprintln!("\n{failed} test(s) failed");
        1
    } else {
        eprintln!("\nall tests passed");
        0
    }
}

fn f49() -> bool {
    use crate::{Alert, Config, Dataset, FraudType, LaborDetector, RuleId};

    let cfg = Config::default();
    assert!(cfg.labor_variance_threshold_pct > 0.0);

    let labor = LaborDetector::new(15.0);
    let ds = Dataset::default();
    let alerts = labor.run(&ds);
    assert!(alerts.is_empty());

    let alert = Alert {
        fraud_type: FraudType::LaborCategory,
        rule_id: RuleId::LaborVariance,
        severity: 5,
        confidence: 85,
        summary: "test".to_string(),
        contract_id: Some("C1".to_string()),
        employee_id: Some("E1".to_string()),
        cage_code: None,
        agency: None,
        predicate_acts: None,
        timestamp: Some("2026-01-01T00:00:00Z".to_string()),
    };
    let json = serde_json::to_string(&alert).unwrap();
    assert!(json.contains("labor_category"));
    assert!(json.contains("LABOR_VARIANCE"));

    true
}

fn f50() -> bool {
    use tempfile::TempDir;

    let tmp = TempDir::new().unwrap();
    let p = tmp.path();

    let contracts = serde_json::json!([{"id":"C1","cage_code":null,"agency":null,"labor_cats":{"Senior":"BA"}}]);
    std::fs::write(p.join("contracts.json"), contracts.to_string()).unwrap();

    let employees = serde_json::json!([{"id":"E1","quals":["BA"],"labor_cat_min":"Junior","verified":false}]);
    std::fs::write(p.join("employees.json"), employees.to_string()).unwrap();

    let labor = serde_json::json!([{"contract_id":"C1","employee_id":"E1","labor_cat":"Principal","hours":40.0,"rate":150.0}]);
    std::fs::write(p.join("labor_charges.json"), labor.to_string()).unwrap();

    let billing = serde_json::json!([{"contract_id":"C1","employee_id":"E99","billed_hours":10.0,"billed_cat":"Junior","period":"2026-01"}]);
    std::fs::write(p.join("billing_records.json"), billing.to_string()).unwrap();

    let ds = crate::Ingest::load_from_path(p).unwrap();
    assert_eq!(ds.contracts.len(), 1);
    assert_eq!(ds.employees.len(), 1);
    assert_eq!(ds.labor_charges.len(), 1);
    assert_eq!(ds.billing_records.len(), 1);

    let labor_det = crate::LaborDetector::new(15.0);
    let ghost_det = crate::GhostDetector::new();
    let labor_alerts = labor_det.run(&ds);
    let ghost_alerts = ghost_det.run(&ds);
    assert!(!labor_alerts.is_empty());
    assert!(labor_alerts.iter().any(|a| format!("{:?}", a.rule_id).contains("LaborQualBelow")));
    assert!(!ghost_alerts.is_empty());
    assert!(ghost_alerts.iter().any(|a| format!("{:?}", a.rule_id).contains("GhostNoEmployee")));

    true
}

fn f51() -> bool {
    let fixtures = fixtures_path();
    assert!(fixtures.exists(), "f51: fixtures dir required");
    let out = Command::new(release_bin())
        .arg("--data-path")
        .arg(&fixtures)
        .output()
        .unwrap();
    assert!(out.status.success() || out.status.code() == Some(1));
    let stdout = String::from_utf8(out.stdout).unwrap();
    let stderr = String::from_utf8(out.stderr).unwrap();
    if stdout.is_empty() && !out.status.success() {
        eprintln!("f51 stderr: {stderr}");
    }
    if !stdout.is_empty() {
        let parsed: Result<Vec<serde_json::Value>, _> = serde_json::from_str(&stdout);
        assert!(parsed.is_ok(), "f51: stdout should be valid JSON array");
    }
    true
}

fn f52() -> bool {
    let fixtures = fixtures_path();
    assert!(fixtures.exists(), "f52: fixtures dir required");
    let (out, stdout, _) = run_bin(&["--data-path", fixtures.to_str().unwrap()]);
    assert!(out.status.code() == Some(1));
    let parsed: Vec<serde_json::Value> = serde_json::from_str(&stdout).unwrap();
    assert!(!parsed.is_empty());
    true
}

fn f53() -> bool {
    let fixtures = fixtures_path();
    assert!(fixtures.exists(), "f53: fixtures dir required");
    let (_, stdout, _) = run_bin(&[
        "--data-path",
        fixtures.to_str().unwrap(),
        "--min-confidence",
        "99",
    ]);
    let parsed: Vec<serde_json::Value> = serde_json::from_str(&stdout).unwrap();
    for a in &parsed {
        assert!(a["confidence"].as_u64().unwrap_or(0) >= 99);
    }
    true
}

fn f54() -> bool {
    let tmp = tempfile::TempDir::new().unwrap();
    std::fs::write(tmp.path().join("contracts.json"), "[]").unwrap();
    let (out, _, _) = run_bin(&["ingest", "--data-path", tmp.path().to_str().unwrap()]);
    assert!(out.status.success());
    true
}

fn f55() -> bool {
    let (out, _, stderr) = run_bin(&["run"]);
    assert!(!out.status.success());
    assert!(stderr.contains("data_path") || stderr.contains("error"));
    true
}

fn f56() -> bool {
    let fixtures = fixtures_path();
    assert!(fixtures.exists(), "f56: fixtures dir required");
    let (_, stdout, _) = run_bin(&[
        "--data-path",
        fixtures.to_str().unwrap(),
        "--agency",
        "DoD",
    ]);
    let parsed: Vec<serde_json::Value> = serde_json::from_str(&stdout).unwrap();
    for a in &parsed {
        assert_eq!(a["agency"].as_str(), Some("DoD"));
    }
    true
}

fn f57() -> bool {
    let fixtures = fixtures_path();
    assert!(fixtures.exists(), "f57: fixtures dir required");
    let (_, stdout, _) = run_bin(&[
        "--data-path",
        fixtures.to_str().unwrap(),
        "--output",
        "csv",
    ]);
    assert!(stdout.contains("fraud_type"));
    assert!(stdout.contains("confidence"));
    assert!(stdout.lines().count() >= 2);
    true
}

fn f58() -> bool {
    let fixtures = fixtures_path();
    assert!(fixtures.exists(), "f58: fixtures dir required");
    let tmp = tempfile::TempDir::new().unwrap();
    let p = tmp.path().join("ref.json");
    let (out, _, _) = run_bin(&[
        "--data-path",
        fixtures.to_str().unwrap(),
        "export-referral",
        "--path",
        p.to_str().unwrap(),
    ]);
    assert!(out.status.code() == Some(0) || out.status.code() == Some(1));
    assert!(p.exists());
    let content = std::fs::read_to_string(&p).unwrap();
    let parsed: serde_json::Value = serde_json::from_str(&content).unwrap();
    assert!(parsed["document_type"].as_str().unwrap().contains("DoD"));
    true
}

fn f59() -> bool {
    let fixtures = fixtures_path();
    assert!(fixtures.exists(), "f59: fixtures dir required");
    let tmp = tempfile::TempDir::new().unwrap();
    let p = tmp.path().join("fbi.json");
    let (out, _, _) = run_bin(&[
        "--data-path",
        fixtures.to_str().unwrap(),
        "export-referral",
        "--fbi",
        "--path",
        p.to_str().unwrap(),
    ]);
    assert!(out.status.code() == Some(0) || out.status.code() == Some(1));
    let content = std::fs::read_to_string(&p).unwrap();
    let parsed: serde_json::Value = serde_json::from_str(&content).unwrap();
    assert!(parsed["document_type"].as_str().unwrap().contains("FBI"));
    true
}

fn f60() -> bool {
    let tmp = tempfile::TempDir::new().unwrap();
    std::fs::write(tmp.path().join("contracts.json"), "[]").unwrap();
    std::fs::write(tmp.path().join("employees.json"), "[]").unwrap();
    std::fs::write(tmp.path().join("labor_charges.json"), "[]").unwrap();
    std::fs::write(tmp.path().join("billing_records.json"), "[]").unwrap();
    let (out, stdout, _) = run_bin(&["--data-path", tmp.path().to_str().unwrap()]);
    assert!(out.status.success());
    assert_eq!(stdout.trim(), "[]");
    true
}
