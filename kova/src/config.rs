// Copyright (c) 2026 The Cochran Block. All rights reserved.
//! Config. ~/.kova/config.toml. Paths. Bootstrap. Build presets.
//! f78=model_path_for_role

use std::path::{Path, PathBuf};

#[derive(serde::Deserialize, Default)]
struct ConfigFile {
    #[serde(default)]
    paths: PathsSection,
    #[serde(default)]
    models: ModelsSection,
    #[serde(default)]
    orchestration: OrchestrationSection,
    #[serde(default)]
    build: BuildSection,
    #[serde(default)]
    cursor: CursorSection,
    #[serde(default)]
    inference: InferenceSection,
}

#[derive(serde::Deserialize, Default)]
struct InferenceSection {
    #[serde(default = "default_model_cache_size")]
    model_cache_size: usize,
    #[serde(default)]
    model_idle_unload_secs: Option<u64>,
    #[serde(default = "default_router_structured")]
    router_structured: bool,
    #[serde(default)]
    code_gen_structured: bool,
}

fn default_router_structured() -> bool {
    true
}

fn default_model_cache_size() -> usize {
    2
}

#[derive(serde::Deserialize)]
struct CursorSection {
    #[serde(default = "default_prompts_enabled")]
    prompts_enabled: bool,
}

impl Default for CursorSection {
    fn default() -> Self {
        Self {
            prompts_enabled: default_prompts_enabled(),
        }
    }
}

fn default_prompts_enabled() -> bool {
    true
}

#[derive(serde::Deserialize, Default)]
struct BuildSection {
    #[serde(default)]
    workspace_root: Option<String>,
    #[serde(default)]
    presets: std::collections::HashMap<String, T88>,
}

/// t88=BuildPreset. Package, target, features for correct cargo invocations.
#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
pub struct T88 {
    pub package: String,
    #[serde(default)]
    pub target: Option<String>,
    #[serde(default)]
    pub features: Vec<String>,
    #[serde(default)]
    pub target_dir_in_project: bool,
}

#[derive(serde::Deserialize, Default)]
struct PathsSection {
    project: Option<String>,
    /// Root for project discovery. KOVA_PROJECTS_ROOT overrides. Scans workspace members + sibling Cargo.toml dirs.
    #[serde(default)]
    projects_root: Option<String>,
}

#[derive(serde::Deserialize, Default)]
struct ModelsSection {
    router: Option<String>,
    coder: Option<String>,
    fix: Option<String>,
}

#[derive(serde::Deserialize)]
struct OrchestrationSection {
    #[serde(default = "default_router_resident")]
    router_resident: bool,
    #[serde(default = "default_idle_unload")]
    specialist_idle_unload_secs: u64,
    #[serde(default = "default_max_fix_retries")]
    max_fix_retries: u32,
    #[serde(default = "default_run_clippy")]
    run_clippy: bool,
}

impl Default for OrchestrationSection {
    fn default() -> Self {
        Self {
            router_resident: default_router_resident(),
            specialist_idle_unload_secs: default_idle_unload(),
            max_fix_retries: default_max_fix_retries(),
            run_clippy: default_run_clippy(),
        }
    }
}

fn default_router_resident() -> bool {
    true
}
fn default_idle_unload() -> u64 {
    300
}
fn default_max_fix_retries() -> u32 {
    2
}
fn default_run_clippy() -> bool {
    true
}

/// t89=ModelRole. Router, Coder, Fix.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum T89 {
    Router,
    Coder,
    Fix,
}

fn expand_path(s: &str) -> PathBuf {
    let s = s.trim();
    if s == "~" {
        home()
    } else if let Some(rest) = s.strip_prefix("~/") {
        home().join(rest)
    } else {
        PathBuf::from(s)
    }
}

fn load_config() -> ConfigFile {
    let path = kova_dir().join("config.toml");
    std::fs::read_to_string(&path)
        .ok()
        .and_then(|s| toml::from_str(&s).ok())
        .unwrap_or_default()
}

/// f78=model_path_for_role. Path for a model role. Env > config > default.
/// f78=model_path_for_role
pub fn f78(role: T89) -> Option<PathBuf> {
    let env_key = match role {
        T89::Router => "KOVA_MODEL_ROUTER",
        T89::Coder => "KOVA_MODEL_CODER",
        T89::Fix => "KOVA_MODEL_FIX",
    };
    if let Ok(p) = std::env::var(env_key) {
        let path = expand_path(&p);
        if path.exists() {
            return Some(path);
        }
    }
    let cfg = load_config();
    if let Some(name) = match role {
        T89::Router => cfg.models.router.as_deref(),
        T89::Coder => cfg.models.coder.as_deref(),
        T89::Fix => cfg.models.fix.as_deref(),
    } {
        let path = expand_path(name);
        if path.exists() {
            return Some(path);
        }
    }
    let default = models_dir().join(default_filename(role));
    if default.exists() {
        Some(default)
    } else {
        None
    }
}

fn default_filename(role: T89) -> &'static str {
    match role {
        T89::Router | T89::Fix => "Qwen2.5-Coder-0.5B-Instruct-Q4_K_M.gguf",
        T89::Coder => "Qwen2.5-Coder-0.5B-Instruct-Q4_K_M.gguf",
    }
}

/// Orchestration settings from config.
pub fn orchestration_router_resident() -> bool {
    load_config().orchestration.router_resident
}

pub fn orchestration_specialist_idle_unload_secs() -> u64 {
    load_config().orchestration.specialist_idle_unload_secs
}

/// Model cache: max models in memory. KOVA_MODEL_CACHE_SIZE or config [inference] model_cache_size.
pub fn model_cache_size() -> usize {
    std::env::var("KOVA_MODEL_CACHE_SIZE")
        .ok()
        .and_then(|s| s.parse().ok())
        .filter(|n: &usize| *n > 0)
        .unwrap_or_else(|| load_config().inference.model_cache_size.max(1))
}

/// Use grammar-constrained output for Coder. Experimental. Config [inference] code_gen_structured.
pub fn code_gen_structured() -> bool {
    std::env::var("KOVA_CODE_GEN_STRUCTURED")
        .ok()
        .map(|s| s != "0" && s != "false" && s != "no")
        .unwrap_or_else(|| load_config().inference.code_gen_structured)
}

/// Use grammar-constrained output for Router. Config [inference] router_structured.
pub fn router_structured() -> bool {
    std::env::var("KOVA_ROUTER_STRUCTURED")
        .ok()
        .map(|s| s != "0" && s != "false" && s != "no")
        .unwrap_or_else(|| load_config().inference.router_structured)
}

/// Model idle unload: secs before evicting. Config [inference] model_idle_unload_secs or orchestration.
pub fn model_idle_unload_secs() -> u64 {
    load_config()
        .inference
        .model_idle_unload_secs
        .unwrap_or_else(orchestration_specialist_idle_unload_secs)
}

pub fn orchestration_max_fix_retries() -> u32 {
    load_config().orchestration.max_fix_retries
}

pub fn orchestration_run_clippy() -> bool {
    load_config().orchestration.run_clippy
}

/// Cursor prompts enabled. When false, no Cursor rules/skills injected.
pub fn cursor_prompts_enabled() -> bool {
    load_config().cursor.prompts_enabled
}

/// f97=home. HOME env or /.
pub fn home() -> PathBuf {
    std::env::var("HOME")
        .map(PathBuf::from)
        .unwrap_or_else(|_| PathBuf::from("/"))
}

/// f98=kova_dir. ~/.kova.
pub fn kova_dir() -> PathBuf {
    home().join(".kova")
}

/// f99=prompts_dir. ~/.kova/prompts.
pub fn prompts_dir() -> PathBuf {
    kova_dir().join("prompts")
}

/// f100=sled_path. ~/.kova/sled.db.
pub fn sled_path() -> PathBuf {
    kova_dir().join("sled.db")
}

/// f101=models_dir. ~/.kova/models.
pub fn models_dir() -> PathBuf {
    kova_dir().join("models")
}

/// f102=inference_model_path. Model path for inference. KOVA_INFERENCE_MODEL or f78(Coder) or default.
pub fn inference_model_path() -> Option<PathBuf> {
    std::env::var("KOVA_INFERENCE_MODEL")
        .ok()
        .map(|s| expand_path(&s))
        .filter(|p| p.exists())
        .or_else(|| f78(T89::Coder))
}

/// f94=default_project. Default project for execution. KOVA_PROJECT > config [paths] project > cwd.
pub fn default_project() -> PathBuf {
    if let Ok(p) = std::env::var("KOVA_PROJECT") {
        let path = expand_path(&p);
        if path.exists() {
            return path;
        }
    }
    if let Some(name) = load_config().paths.project.as_deref() {
        let path = expand_path(name);
        if path.exists() {
            return path;
        }
    }
    std::env::current_dir().unwrap_or_else(|_| home())
}

/// Root for project discovery. KOVA_PROJECTS_ROOT > config [paths] projects_root > home.
fn projects_root() -> PathBuf {
    if let Ok(p) = std::env::var("KOVA_PROJECTS_ROOT") {
        let path = expand_path(&p);
        if path.exists() {
            return path;
        }
    }
    if let Some(name) = load_config().paths.projects_root.as_deref() {
        let path = expand_path(name);
        if path.exists() {
            return path;
        }
    }
    home()
}

/// f95=discover_projects. Discover all projects under projects_root. Workspace members + sibling dirs with Cargo.toml. Sorted by name.
pub fn discover_projects() -> Vec<PathBuf> {
    let root = projects_root();
    let mut seen = std::collections::HashSet::new();
    let mut out = Vec::new();

    // 1. Parse root Cargo.toml for [workspace] members
    let manifest = root.join("Cargo.toml");
    if manifest.exists() {
        if let Ok(content) = std::fs::read_to_string(&manifest) {
            if let Ok(toml) = content.parse::<toml::Value>() {
                if let Some(ws) = toml.get("workspace") {
                    if let Some(members) = ws.get("members") {
                        let members = match members {
                            toml::Value::Array(a) => a
                                .iter()
                                .filter_map(|v| v.as_str())
                                .map(|s| s.to_string())
                                .collect::<Vec<_>>(),
                            toml::Value::String(s) => vec![s.clone()],
                            _ => vec![],
                        };
                        for m in members {
                            let full = root.join(&m);
                            if full.exists() && full.join("Cargo.toml").exists() && seen.insert(full.clone()) {
                                out.push(full);
                            }
                        }
                    }
                }
            }
        }
    }

    // 2. Scan immediate subdirs for Cargo.toml (rogue-repo, ronin-sites, kova-daemon, etc.)
    if let Ok(entries) = std::fs::read_dir(&root) {
        for e in entries.flatten() {
            let path = e.path();
            if path.is_dir() {
                let name = path.file_name().and_then(|n| n.to_str()).unwrap_or("");
                if name.starts_with('.') || name == "target" || name == "vendor" {
                    continue;
                }
                if path.join("Cargo.toml").exists() && seen.insert(path.clone()) {
                    out.push(path);
                }
            }
        }
    }

    out.sort_by(|a, b| {
        a.file_name()
            .and_then(|n| n.to_str())
            .unwrap_or("")
            .cmp(b.file_name().and_then(|n| n.to_str()).unwrap_or(""))
    });
    out
}

/// f103=backlog_path. KOVA_BACKLOG_PATH or ~/.kova/backlog.json.
pub fn backlog_path() -> PathBuf {
    std::env::var("KOVA_BACKLOG_PATH")
        .map(PathBuf::from)
        .unwrap_or_else(|_| kova_dir().join("backlog.json"))
}

/// f104=workspace_root. Workspace root for builds. KOVA_WORKSPACE_ROOT > config [build] workspace_root > detect from project.
pub fn workspace_root(project: &Path) -> PathBuf {
    if let Ok(p) = std::env::var("KOVA_WORKSPACE_ROOT") {
        let path = expand_path(&p);
        if path.exists() {
            return path;
        }
    }
    if let Some(s) = load_config().build.workspace_root.as_deref() {
        let path = expand_path(s);
        if path.exists() {
            return path;
        }
    }
    workspace_root_from_project(project)
}

/// Walk up from project to find Cargo.toml with [workspace]. Fallback: project.
fn workspace_root_from_project(project: &Path) -> PathBuf {
    let mut dir = project.to_path_buf();
    if dir.is_file() {
        dir = dir.parent().unwrap_or(&dir).to_path_buf();
    }
    while !dir.as_os_str().is_empty() {
        let manifest = dir.join("Cargo.toml");
        if manifest.exists() {
            if let Ok(content) = std::fs::read_to_string(&manifest) {
                if content.contains("[workspace]") {
                    return dir;
                }
            }
        }
        dir = match dir.parent() {
            Some(p) => p.to_path_buf(),
            None => break,
        };
    }
    project.to_path_buf()
}

/// Load build preset by project name (e.g. "oakilydokily", "mural-wasm").
/// f105=load_build_preset
pub fn load_build_preset(project_name: &str) -> Option<T88> {
    load_config().build.presets.get(project_name).cloned()
}

/// All build presets for API.
/// f106=all_build_presets
pub fn all_build_presets() -> std::collections::HashMap<String, T88> {
    load_config().build.presets
}

/// Infer preset name from project path. e.g. .../oakilydokily -> "oakilydokily", .../mural-wasm -> "mural-wasm".
/// f107=infer_preset_name
pub fn infer_preset_name(project: &Path) -> Option<String> {
    let name = project.file_name()?.to_str()?;
    if name.is_empty() || name == "." || name == ".." {
        return None;
    }
    Some(name.to_string())
}

/// Type aliases for ergonomics. Token names: T88, T89.
pub use T88 as BuildPreset;
pub use T89 as ModelRole;

/// f108=bind_addr. KOVA_BIND or 127.0.0.1:3002.
pub fn bind_addr() -> std::net::SocketAddr {
    std::env::var("KOVA_BIND")
        .ok()
        .and_then(|s| s.parse().ok())
        .unwrap_or_else(|| std::net::SocketAddr::from(([127, 0, 0, 1], 3002u16)))
}

/// Skip TLS when binding to loopback. Localhost = no TLS = fastest path.
/// Set KOVA_FAST_LOCALHOST=false to disable (e.g. when adding TLS for remote).
pub fn fast_localhost() -> bool {
    std::env::var("KOVA_FAST_LOCALHOST")
        .ok()
        .map(|s| s != "0" && s != "false" && s != "no")
        .unwrap_or(true)
}

/// f109=bootstrap. Bootstrap ~/.kova, prompts/, config.toml, default prompts.
pub fn bootstrap() -> anyhow::Result<()> {
    let dir = kova_dir();
    std::fs::create_dir_all(&dir)?;
    std::fs::create_dir_all(prompts_dir())?;
    std::fs::create_dir_all(models_dir())?;

    let config_path = dir.join("config.toml");
    if !config_path.exists() {
        let default = r#"[paths]
backlog = "~/.kova/backlog.json"
# project = "~/my-project"  # default project for code gen; KOVA_PROJECT overrides
# projects_root = "~"  # root for discovery; KOVA_PROJECTS_ROOT overrides. Scans workspace + sibling Cargo.toml dirs.

[serve]
bind = "127.0.0.1:3002"

[models]
router = "~/.kova/models/Qwen2.5-Coder-0.5B-Instruct-Q4_K_M.gguf"
coder = "~/.kova/models/Qwen2.5-Coder-0.5B-Instruct-Q4_K_M.gguf"
fix = "~/.kova/models/Qwen2.5-Coder-0.5B-Instruct-Q4_K_M.gguf"

[orchestration]
router_resident = true
specialist_idle_unload_secs = 300
# max_fix_retries: DDI (Debugging Decay Index) — fix loop loses effectiveness after 2–3 attempts
max_fix_retries = 2
run_clippy = true

[cursor]
prompts_enabled = true

[inference]
# model_cache_size = 2        # Max models in memory
# model_idle_unload_secs = 300  # Unload after N sec idle
# router_structured = true    # Use grammar for Router (Phase 2)
# code_gen_structured = false # Experimental: structured code output

[build]
# workspace_root = "~"  # KOVA_WORKSPACE_ROOT overrides

[build.presets.oakilydokily]
package = "oakilydokily"
features = ["approuter"]

[build.presets.mural-wasm]
package = "mural-wasm"
target = "wasm32-unknown-unknown"

[build.presets.kova]
package = "kova"
features = ["serve"]
target = "aarch64-apple-darwin"
target_dir_in_project = true

[build.presets.approuter]
package = "approuter"

[build.presets.cochranblock]
package = "cochranblock"

[build.presets.rogue-repo]
package = "rogue-repo"
"#;
        std::fs::write(&config_path, default)?;
    }

    let prompts = prompts_dir();
    let system_path = prompts.join("system.md");
    if !system_path.exists() {
        let default = r#"# System prompt

You are Kova, an augment engine. Execute intent. Build fast, ship fast.

## Style (P12)
- Voice: short, active, concrete
- Avoid: utilize, leverage, facilitate, enhance, optimize, comprehensive, holistic, robust, seamlessly, empower, streamline, synergy, paradigm
- Use: use, apply, let, enable, improve, tune, full, drop, simplify

## Principles
- Intent-first. Why not how.
- Verify. No fire-and-forget.
- Project separation. One rebuild at a time.
"#;
        std::fs::write(&system_path, default)?;
    }

    let persona_path = prompts.join("persona.md");
    if !persona_path.exists() {
        let default = r#"# Persona

You are a Senior Systems Architect. Execute my intent. Direct, precise, efficient.
"#;
        std::fs::write(&persona_path, default)?;
    }

    let backlog_path = dir.join("backlog.json");
    if !backlog_path.exists() {
        std::fs::write(&backlog_path, r#"{"items":[]}"#)?;
    }

    Ok(())
}

/// f110=load_prompt
pub fn load_prompt(name: &str) -> String {
    let path = prompts_dir().join(format!("{}.md", name));
    std::fs::read_to_string(&path).unwrap_or_default()
}

#[cfg(test)]
mod tests {
    use super::*;

    /// f78=model_path_for_role
    #[test]
    fn model_role_default_filename() {
        assert_eq!(
            default_filename(T89::Router),
            "Qwen2.5-Coder-0.5B-Instruct-Q4_K_M.gguf"
        );
        assert_eq!(
            default_filename(T89::Coder),
            "Qwen2.5-Coder-0.5B-Instruct-Q4_K_M.gguf"
        );
        assert_eq!(
            default_filename(T89::Fix),
            "Qwen2.5-Coder-0.5B-Instruct-Q4_K_M.gguf"
        );
    }

    /// expand_path (f97 home)
    #[test]
    fn expand_path_tilde() {
        let h = home();
        let p = super::expand_path("~/foo/bar");
        assert_eq!(p, h.join("foo/bar"));
    }
}
