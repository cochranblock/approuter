// Copyright (c) 2026 The Cochran Block. All rights reserved.
//! Demo mode: action script recording for UI testing.
//! Record user actions + function invocations; replay to iterate Kova dev cycles.

use serde::{Deserialize, Serialize};
use std::path::Path;

/// Single recorded action. Web: click, input, api_call. Egui: click, key, intent.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "kind", rename_all = "snake_case")]
pub enum DemoAction {
    /// Web: element click. selector, timestamp.
    WebClick { selector: String, ts_ms: u64 },
    /// Web: input change. id, value.
    WebInput { id: String, value: String },
    /// Web or egui: API/intent invoked. method, path_or_intent, body_summary.
    ApiCall {
        method: String,
        path: String,
        #[serde(skip_serializing_if = "Option::is_none")]
        body_summary: Option<String>,
    },
    /// Egui: chat message sent.
    EguiSend { text: String },
    /// Egui: intent confirmed (y/n).
    EguiConfirm { intent: String },
    /// Screenshot captured. path relative to demo dir.
    Screenshot { path: String },
}

/// Demo recording session. Written to ~/.kova/demos/{name}.json
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DemoRecord {
    pub name: String,
    pub source: String, // "web" | "egui"
    pub actions: Vec<DemoAction>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub started_at: Option<String>,
}

impl DemoRecord {
    pub fn new(name: impl Into<String>, source: &str) -> Self {
        Self {
            name: name.into(),
            source: source.to_string(),
            actions: Vec::new(),
            started_at: Some(chrono_now()),
        }
    }

    pub fn push(&mut self, action: DemoAction) {
        self.actions.push(action);
    }

    pub fn save(&self, out_dir: &Path) -> Result<std::path::PathBuf, String> {
        std::fs::create_dir_all(out_dir).map_err(|e| e.to_string())?;
        let safe = self.name.replace(['/', '\\', ':'], "_");
        let path = out_dir.join(format!("{}.json", safe));
        let json = serde_json::to_string_pretty(self).map_err(|e| e.to_string())?;
        std::fs::write(&path, json).map_err(|e| e.to_string())?;
        Ok(path)
    }

    pub fn load(path: &Path) -> Result<Self, String> {
        let s = std::fs::read_to_string(path).map_err(|e| e.to_string())?;
        serde_json::from_str(&s).map_err(|e| e.to_string())
    }
}

fn chrono_now() -> String {
    use std::time::{SystemTime, UNIX_EPOCH};
    let t = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_secs();
    format!("{}", t)
}

/// Demo dir: ~/.kova/demos or KOVA_DEMO_DIR
pub fn demo_dir() -> std::path::PathBuf {
    std::env::var("KOVA_DEMO_DIR")
        .ok()
        .map(std::path::PathBuf::from)
        .unwrap_or_else(|| {
            dirs::home_dir()
                .unwrap_or_else(|| std::path::PathBuf::from("."))
                .join(".kova")
                .join("demos")
        })
}
