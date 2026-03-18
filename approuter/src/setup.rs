//! f114–f137 setup commands. Stub until full implementation restored.

// Unlicense — cochranblock.org
// Contributors: Mattbusel (XFactor), GotEmCoach, KOVA, Claude Opus 4.6, SuperNinja, Composer 1.5, Google Gemini Pro 3

use std::path::Path;

/// cb_root = cochranblock root. Default: current dir.
pub fn cb_root() -> std::path::PathBuf {
    std::env::current_dir().unwrap_or_else(|_| std::path::PathBuf::from("."))
}

pub fn f114(_root: &Path) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    Err("setup::f114 not implemented".into())
}
pub fn f117(_root: &Path) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    Err("setup::f117 not implemented".into())
}
pub fn f118(_root: &Path) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    Err("setup::f118 not implemented".into())
}
pub fn f119(_root: &Path) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    Err("setup::f119 not implemented".into())
}
pub fn f120(_root: &Path) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    Err("setup::f120 not implemented".into())
}
pub fn f121(_root: &Path) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    Err("setup::f121 not implemented".into())
}
pub fn f122(_root: &Path, _domain: &str) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    Err("setup::f122 not implemented".into())
}
pub fn f123(_root: &Path, _domain: &str) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    Err("setup::f123 not implemented".into())
}
pub fn f124(_root: &Path, _domain: &str, _value: &str) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    Err("setup::f124 not implemented".into())
}
pub fn f125(_root: &Path, _domain: &str, _name: &str, _target: &str) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    Err("setup::f125 not implemented".into())
}
pub fn f132(_root: &Path, _package: Option<&str>) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    Err("setup::f132 not implemented".into())
}
pub fn f133(_root: &Path) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    Err("setup::f133 not implemented".into())
}
pub fn f134(_root: &Path, _target: &std::path::Path, _force: bool) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    Err("setup::f134 not implemented".into())
}
pub fn f135(_root: &Path, _project: Option<&str>, _sa_name: Option<&str>, _key_file: Option<&str>) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    Err("setup::f135 not implemented".into())
}
pub fn f136(_free_only: bool, _preferred: bool) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    Err("setup::f136 not implemented".into())
}
pub fn f137(_site: &str, _sitemap: &str) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    Err("setup::f137 not implemented".into())
}