// Copyright (c) 2026 The Cochran Block. All rights reserved.
#![allow(non_camel_case_types, non_snake_case, dead_code)]
//! Compute layer. Execute plan. f15. Uses build presets when available.

use crate::config::{infer_preset_name, load_build_preset, workspace_root};
use crate::plan::{t3, t5};
use std::process::Command;

/// t6 = Executor.
#[derive(Debug, Default)]
pub struct t6;

/// t7 = ActionResult. s10=name, s11=ok, s13=error.
#[derive(Debug, Clone)]
pub struct t7 {
    pub s10: String,
    pub s11: bool,
    pub s13: String,
}

impl t6 {
    /// f15 = execute. Run plan, return results.
    pub fn f15(&self, plan: &t3) -> anyhow::Result<Vec<t7>> {
        let mut out = Vec::new();
        for step in &plan.s3 {
            let r = self.f16(&step.s6, plan, &plan.s5);
            out.push(r);
        }
        Ok(out)
    }

    fn f16(
        &self,
        action: &t5,
        plan: &t3,
        approuter_dir: &Option<std::path::PathBuf>,
    ) -> t7 {
        let project = &plan.s4;
        match action {
            t5::CargoCheck => self.run_cargo_check(plan),
            t5::CargoBuild { release } => self.run_cargo_build(plan, *release),
            t5::CargoTest => {
                // f93=cargo_test via run_cargo (main binary: no exopack)
                let o = Command::new("cargo")
                    .arg("test")
                    .current_dir(project)
                    .output();
                let (ok, stderr) = match o {
                    Ok(o) => (o.status.success(), String::from_utf8_lossy(&o.stderr).into_owned()),
                    Err(e) => (false, e.to_string()),
                };
                t7 {
                    s10: "cargo test".into(),
                    s11: ok,
                    s13: stderr,
                }
            }
            t5::ApprouterUpdateTunnel => {
                let Some(ref ad) = approuter_dir else {
                    return t7 {
                        s10: "approuter --update-tunnel".into(),
                        s11: false,
                        s13: "approuter_dir not set".into(),
                    };
                };
                let r = Command::new("approuter")
                    .arg("--update-tunnel")
                    .current_dir(ad)
                    .output();
                match r {
                    Ok(o) => t7 {
                        s10: "approuter --update-tunnel".into(),
                        s11: o.status.success(),
                        s13: if o.status.success() {
                            String::new()
                        } else {
                            String::from_utf8_lossy(&o.stderr).into()
                        },
                    },
                    Err(e) => t7 {
                        s10: "approuter --update-tunnel".into(),
                        s11: false,
                        s13: e.to_string(),
                    },
                }
            }
            t5::ApprouterSetupRoguerepo => {
                let Some(ref ad) = approuter_dir else {
                    return t7 {
                        s10: "approuter --setup-roguerepo".into(),
                        s11: false,
                        s13: "approuter_dir not set".into(),
                    };
                };
                let r = Command::new("approuter")
                    .arg("--setup-roguerepo")
                    .current_dir(ad)
                    .output();
                match r {
                    Ok(o) => t7 {
                        s10: "approuter --setup-roguerepo".into(),
                        s11: o.status.success(),
                        s13: if o.status.success() {
                            String::new()
                        } else {
                            String::from_utf8_lossy(&o.stderr).into()
                        },
                    },
                    Err(e) => t7 {
                        s10: "approuter --setup-roguerepo".into(),
                        s11: false,
                        s13: e.to_string(),
                    },
                }
            }
            t5::Custom { cmd, args } => {
                let r = Command::new(cmd)
                    .args(args)
                    .current_dir(project)
                    .output();
                match r {
                    Ok(o) => t7 {
                        s10: format!("{} {}", cmd, args.join(" ")),
                        s11: o.status.success(),
                        s13: if o.status.success() {
                            String::new()
                        } else {
                            String::from_utf8_lossy(&o.stderr).into()
                        },
                    },
                    Err(e) => t7 {
                        s10: cmd.clone(),
                        s11: false,
                        s13: e.to_string(),
                    },
                }
            }
        }
    }

    fn run_cargo_check(&self, plan: &t3) -> t7 {
        let (cwd, args) = self.cargo_cwd_args(plan, "check", &[]);
        let r = Command::new("cargo")
            .args(args)
            .current_dir(cwd)
            .output();
        self.cargo_result("cargo check", r)
    }

    fn run_cargo_build(&self, plan: &t3, release: bool) -> t7 {
        let extra: Vec<String> = if release {
            vec!["--release".into()]
        } else {
            vec![]
        };
        let (cwd, args) = self.cargo_cwd_args(plan, "build", &extra);
        let mut cmd = Command::new("cargo");
        cmd.args(&args).current_dir(&cwd);
        if let Some(preset) = self.resolve_preset(plan) {
            if preset.target_dir_in_project {
                let target_dir = plan.s4.join("target");
                cmd.env("CARGO_TARGET_DIR", &target_dir);
            }
        }
        let r = cmd.output();
        self.cargo_result("cargo build", r)
    }

    fn resolve_preset(&self, plan: &t3) -> Option<crate::config::BuildPreset> {
        let name = plan
            .s7
            .clone()
            .or_else(|| infer_preset_name(&plan.s4))?;
        load_build_preset(&name)
    }

    fn cargo_cwd_args(
        &self,
        plan: &t3,
        subcmd: &str,
        extra: &[String],
    ) -> (std::path::PathBuf, Vec<String>) {
        let preset = self.resolve_preset(plan);
        if let Some(p) = preset {
            let root = workspace_root(&plan.s4);
            let mut args = vec![subcmd.to_string(), "-p".to_string(), p.package];
            if let Some(t) = &p.target {
                args.push("--target".to_string());
                args.push(t.clone());
            }
            for f in &p.features {
                args.push("--features".to_string());
                args.push(f.clone());
            }
            args.extend(extra.iter().cloned());
            (root, args)
        } else {
            let mut args = vec![subcmd.to_string()];
            args.extend(extra.iter().cloned());
            (plan.s4.clone(), args)
        }
    }

    fn cargo_result(&self, name: &str, r: Result<std::process::Output, std::io::Error>) -> t7 {
        match r {
            Ok(o) => t7 {
                s10: name.to_string(),
                s11: o.status.success(),
                s13: if o.status.success() {
                    String::new()
                } else {
                    String::from_utf8_lossy(&o.stderr).into()
                },
            },
            Err(e) => t7 {
                s10: name.to_string(),
                s11: false,
                s13: e.to_string(),
            },
        }
    }
}
