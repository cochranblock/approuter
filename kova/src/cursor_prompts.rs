// Copyright (c) 2026 The Cochran Block. All rights reserved.
//! Load Cursor prompts for injection into model context.
//! fN=load_cursor_prompts

use std::path::Path;

/// Baked-in rules. Always included when prompts enabled. No external files required.
pub(crate) fn baked_prompts() -> String {
    let blocking = include_str!("../assets/prompts/blocking.mdc");
    let augment = include_str!("../assets/prompts/augment-not-intent.mdc");
    let tokenization = include_str!("../assets/prompts/tokenization.mdc");
    let hosting = include_str!("../assets/prompts/hosting-schematic.mdc");
    let compression = include_str!("../docs/compression_map.md");
    format!(
        "\n\n--- blocking (baked) ---\n{}\n\n--- augment-not-intent (baked) ---\n{}\n\n--- tokenization (baked) ---\n{}\n\n--- hosting-schematic (baked) ---\n{}\n\n--- compression_map (baked) ---\n{}",
        blocking, augment, tokenization, hosting, compression
    )
}

/// f111=load_cursor_prompts. Discover and concatenate all Cursor prompts. Returns empty if disabled.
/// Baked-in rules (blocking, augment, tokenization, compression_map) are always included.
pub fn load_cursor_prompts(workspace_root: &Path) -> String {
    if !crate::cursor_prompts_enabled() {
        return String::new();
    }
    let mut out = baked_prompts();

    // 1. ~/.cursor/rules (append)
    if let Ok(home) = std::env::var("HOME") {
        let rules = Path::new(&home).join(".cursor/rules");
        if rules.exists() {
            if let Ok(entries) = std::fs::read_dir(&rules) {
                for e in entries.flatten() {
                    if e.path().extension().is_some_and(|ext| ext == "mdc") {
                        if let Ok(c) = std::fs::read_to_string(e.path()) {
                            out.push_str(&format!("\n\n--- {} ---\n{}", e.path().display(), c));
                        }
                    }
                }
            }
        }
    }

    // 2. ~/.cursor/shared-rules (append)
    if let Ok(home) = std::env::var("HOME") {
        let shared = Path::new(&home).join(".cursor/shared-rules");
        if shared.exists() {
            if let Ok(entries) = std::fs::read_dir(&shared) {
                for e in entries.flatten() {
                    if e.path().extension().is_some_and(|ext| ext == "mdc") {
                        if let Ok(c) = std::fs::read_to_string(e.path()) {
                            out.push_str(&format!("\n\n--- {} ---\n{}", e.path().display(), c));
                        }
                    }
                }
            }
        }
    }

    // 3. workspace .cursor/rules (append)
    let wr = workspace_root.join(".cursor/rules");
    if wr.exists() {
        if let Ok(entries) = std::fs::read_dir(&wr) {
            for e in entries.flatten() {
                if e.path().extension().is_some_and(|ext| ext == "mdc") {
                    if let Ok(c) = std::fs::read_to_string(e.path()) {
                        out.push_str(&format!("\n\n--- {} ---\n{}", e.path().display(), c));
                    }
                }
            }
        }
    }

    // 4. ~/.cursor/protocols (append)
    if let Ok(home) = std::env::var("HOME") {
        let protocols = Path::new(&home).join(".cursor/protocols");
        if protocols.exists() {
            for name in [
                "PROTOCOLS.mdc",
                "PROTOCOL_COMPLIANCE.md",
                "UNIFIED_PROTOCOLS.md",
                "README.md",
                "env_token_map.md",
            ] {
                let p = protocols.join(name);
                if p.exists() {
                    if let Ok(c) = std::fs::read_to_string(&p) {
                        out.push_str(&format!("\n\n--- {} ---\n{}", p.display(), c));
                    }
                }
            }
        }
    }

    // 4b. All discovered projects: .cursor/rules and .cursor/protocols (append)
    for project in crate::discover_projects() {
        append_project_rules(&project, &mut out);
    }

    // 5. ~/.cursor/skills-cursor
    if let Ok(home) = std::env::var("HOME") {
        let skills = Path::new(&home).join(".cursor/skills-cursor");
        if skills.exists() {
            append_skill_dir(&skills, &mut out);
        }
    }

    // 6. ~/.cursor/skills
    if let Ok(home) = std::env::var("HOME") {
        let skills = Path::new(&home).join(".cursor/skills");
        if skills.exists() {
            append_skill_dir(&skills, &mut out);
        }
    }

    // 7. AGENTS.md
    let agents = workspace_root.join("AGENTS.md");
    if agents.exists() {
        if let Ok(c) = std::fs::read_to_string(&agents) {
            out.push_str(&format!("\n\n--- AGENTS.md ---\n{}", c));
        }
    }

    // 8. protocol_map, compression_map
    for name in ["protocol_map.md", "compression_map.md"] {
        for base in [workspace_root.join("docs"), workspace_root.to_path_buf()] {
            let p = base.join(name);
            if p.exists() {
                if let Ok(c) = std::fs::read_to_string(&p) {
                    out.push_str(&format!("\n\n--- {} ---\n{}", name, c));
                }
                break;
            }
        }
    }

    out
}

fn append_project_rules(project: &Path, out: &mut String) {
    // project/.cursor/rules/*.mdc
    let rules = project.join(".cursor/rules");
    if rules.exists() {
        if let Ok(entries) = std::fs::read_dir(&rules) {
            for e in entries.flatten() {
                if e.path().extension().is_some_and(|ext| ext == "mdc") {
                    if let Ok(c) = std::fs::read_to_string(e.path()) {
                        out.push_str(&format!(
                            "\n\n--- {} (project) ---\n{}",
                            e.path().display(),
                            c
                        ));
                    }
                }
            }
        }
    }
    // project/.cursor/protocols/*.md
    let protocols = project.join(".cursor/protocols");
    if protocols.exists() {
        if let Ok(entries) = std::fs::read_dir(&protocols) {
            for e in entries.flatten() {
                let p = e.path();
                if p.extension().is_some_and(|ext| ext == "md") {
                    if let Ok(c) = std::fs::read_to_string(&p) {
                        out.push_str(&format!(
                            "\n\n--- {} (project protocol) ---\n{}",
                            p.display(),
                            c
                        ));
                    }
                }
            }
        }
    }
}

fn append_skill_dir(dir: &Path, out: &mut String) {
    if let Ok(entries) = std::fs::read_dir(dir) {
        for e in entries.flatten() {
            let skill_md = e.path().join("SKILL.md");
            if skill_md.exists() {
                if let Ok(c) = std::fs::read_to_string(&skill_md) {
                    out.push_str(&format!("\n\n--- {} ---\n{}", skill_md.display(), c));
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Prove baked content exists and matches external-equivalent rules. Tests baked_prompts()
    /// directly so it does not depend on config (prompts_enabled).
    #[test]
    fn baked_prompts_content_equivalent() {
        let out = baked_prompts();
        assert!(!out.is_empty(), "baked prompts must be non-empty");

        // Blocking (P20)
        assert!(out.contains("Blocking Only") || out.contains("P20"), "blocking rule");
        assert!(out.contains("Never background"), "blocking rule");

        // Augment not intent
        assert!(out.contains("augment"), "augment rule");
        assert!(out.contains("intent"), "augment rule");

        // Tokenization
        assert!(out.contains("fN") || out.contains("tN"), "tokenization rule");
        assert!(out.contains("compression_map") || out.contains("traceability"), "tokenization");

        // Compression map (Kova f/t mappings)
        assert!(out.contains("f81") && out.contains("run_code_gen_pipeline"), "compression_map");
        assert!(out.contains("f78") && out.contains("model_path_for_role"), "compression_map");
    }

    /// Prove load_cursor_prompts injects baked content when enabled. Skips if config disables.
    /// Uses temp dir for KOVA_PROJECTS_ROOT to avoid slow discover_projects on home.
    #[test]
    fn load_cursor_prompts_includes_baked_when_enabled() {
        let tmp = tempfile::TempDir::new().unwrap();
        std::env::set_var("KOVA_PROJECTS_ROOT", tmp.path());
        let out = load_cursor_prompts(tmp.path());
        std::env::remove_var("KOVA_PROJECTS_ROOT");
        if crate::cursor_prompts_enabled() {
            assert!(!out.is_empty(), "when enabled, output must include baked content");
            assert!(out.contains("Blocking Only") || out.contains("P20"));
        }
    }

    /// Prove the exact format serve uses would pass baked content to the pipeline.
    #[test]
    fn system_prompt_format_includes_baked() {
        let cursor = baked_prompts();
        let system_prompt = format!("System\n\nPersona\n\n--- Cursor rules ---\n{}", cursor);
        assert!(system_prompt.contains("Blocking Only"), "serve format must include blocking");
        assert!(system_prompt.contains("f81"), "serve format must include compression_map");
    }
}
