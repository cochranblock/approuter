// Copyright (c) 2026 The Cochran Block. All rights reserved.
#![allow(non_camel_case_types, non_snake_case, dead_code)]
//! Plan layer. Intent → action DAG. f14.

use crate::{t0, t1};
use std::path::PathBuf;

/// t3 = Plan. DAG of actions. s3=steps, s4=project, s5=approuter_dir, s7=project_hint.
#[derive(Debug, Clone)]
pub struct t3 {
    pub s3: Vec<t4>,
    pub s4: PathBuf,
    pub s5: Option<PathBuf>,
    pub s7: Option<String>,
}

/// t4 = PlanStep. One action in the plan.
#[derive(Debug, Clone)]
pub struct t4 {
    pub s6: t5,
}

/// t5 = Action. What to run.
#[derive(Debug, Clone)]
pub enum t5 {
    CargoCheck,
    CargoBuild { release: bool },
    CargoTest,
    ApprouterUpdateTunnel,
    ApprouterSetupRoguerepo,
    Custom { cmd: String, args: Vec<String> },
}

impl t3 {
    /// f14 = plan_from_intent. Map intent to action DAG.
    pub fn f14(intent: &t0, project: PathBuf, approuter_dir: Option<PathBuf>) -> Self {
    let approuter_dir = approuter_dir.or_else(|| {
        std::env::var("HOME")
            .ok()
            .map(|h| PathBuf::from(h).join("approuter"))
    });
    let mut s3 = Vec::new();
    match &intent.s0 {
        t1::FullPipeline => {
            s3.push(t4 { s6: t5::CargoCheck });
            s3.push(t4 { s6: t5::CargoTest });
        }
        t1::Test => {
            s3.push(t4 { s6: t5::CargoTest });
        }
        t1::Compile { release, check_only } => {
            if *check_only {
                s3.push(t4 { s6: t5::CargoCheck });
            } else {
                s3.push(t4 {
                    s6: t5::CargoBuild { release: *release },
                });
            }
        }
        t1::TunnelUpdate => {
            if approuter_dir.is_some() {
                s3.push(t4 {
                    s6: t5::ApprouterUpdateTunnel,
                });
            }
        }
        t1::SetupRoguerepo => {
            if approuter_dir.is_some() {
                s3.push(t4 {
                    s6: t5::ApprouterSetupRoguerepo,
                });
            }
        }
        t1::Custom { cmd, args } => {
            s3.push(t4 {
                s6: t5::Custom {
                    cmd: cmd.clone(),
                    args: args.clone(),
                },
            });
        }
        t1::CloudflarePurge | t1::FixWarnings => {}
    }
        t3 {
            s3,
            s4: project,
            s5: approuter_dir,
            s7: intent.s1.clone(),
        }
    }
}
