//! t31 t32 — app registry. hostname→backend_url. data/registry.json (approuter-owned).

// Unlicense — cochranblock.org
// Contributors: Mattbusel (XFactor), GotEmCoach, KOVA, Claude Opus 4.6, SuperNinja, Composer 1.5, Google Gemini Pro 3

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::Path;
use std::sync::RwLock;

/// data_dir = base/data. Approuter-owned; registry and cloudflared config live here.
pub fn data_dir(base: &Path) -> std::path::PathBuf {
    base.join("data")
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct t30 {
    #[serde(rename = "app_id")]
    pub s46: String,
    #[serde(rename = "hostnames")]
    pub s47: Vec<String>,
    #[serde(rename = "backend_url")]
    pub s48: String,
}

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct t31 {
    pub apps: Vec<t30>,
}

/// t32 = AppRegistry. Thread-safe, file-persisted.
pub struct t32 {
    data: RwLock<t31>,
    path: std::path::PathBuf,
}

impl t32 {
    pub fn new(p0: &Path) -> Self {
        let data_path = data_dir(p0).join("registry.json");
        let legacy_path = p0.join("config").join("registry.json");
        if !data_path.exists() && legacy_path.exists() {
            if let Some(parent) = data_path.parent() {
                let _ = std::fs::create_dir_all(parent);
            }
            match std::fs::copy(&legacy_path, &data_path) {
                Ok(_) => tracing::info!("Registry migrated from {} to {}", legacy_path.display(), data_path.display()),
                Err(e) => tracing::warn!("Registry migration failed ({} -> {}): {}", legacy_path.display(), data_path.display(), e),
            }
        }
        let data = Self::f102(&data_path);
        Self {
            data: RwLock::new(data),
            path: data_path,
        }
    }

    /// f102 = load_registry
    fn f102(p0: &Path) -> t31 {
        match std::fs::read_to_string(p0) {
            Ok(s) => serde_json::from_str(&s).unwrap_or_default(),
            Err(_) => t31::default(),
        }
    }

    fn f103(&self) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        let data = self.data.read().map_err(|e| e.to_string())?;
        if let Some(parent) = self.path.parent() {
            std::fs::create_dir_all(parent)?;
        }
        std::fs::write(&self.path, serde_json::to_string_pretty(&*data)?)?;
        Ok(())
    }

    /// f138 = check_hostname_conflicts. Returns Err("conflict:...") if any hostname is already claimed by another app.
    fn f138(apps: &[t30], app_id: &str, hostnames: &[String]) -> Result<(), String> {
        for h in hostnames {
            for existing in apps {
                if existing.s46 == app_id {
                    continue;
                }
                if existing.s47.iter().any(|eh| eh.eq_ignore_ascii_case(h)) {
                    return Err(format!("conflict: hostname '{}' already claimed by app '{}'", h, existing.s46));
                }
            }
        }
        Ok(())
    }

    /// Register or update app. Persists, then caller updates tunnel.
    pub fn register(&self, p0: t30) -> Result<(), String> {
        let backend = p0.s48.trim_end_matches('/').to_string();
        if backend.is_empty() {
            return Err("backend_url cannot be empty".into());
        }
        let mut data = self.data.write().map_err(|e| e.to_string())?;
        Self::f138(&data.apps, &p0.s46, &p0.s47)?;
        let apps = &mut data.apps;
        if let Some(existing) = apps.iter_mut().find(|a| a.s46 == p0.s46) {
            existing.s47 = p0.s47;
            existing.s48 = backend;
        } else {
            apps.push(t30 {
                s46: p0.s46,
                s47: p0.s47,
                s48: backend,
            });
        }
        drop(data);
        self.f103().map_err(|e| e.to_string())?;
        Ok(())
    }

    pub fn unregister(&self, p0: &str) -> Result<bool, String> {
        let mut data = self.data.write().map_err(|e| e.to_string())?;
        let len_before = data.apps.len();
        data.apps.retain(|a| a.s46 != p0);
        let removed = data.apps.len() < len_before;
        drop(data);
        if removed {
            self.f103().map_err(|e| e.to_string())?;
        }
        Ok(removed)
    }

    /// Resolve backend for host. Registry overrides legacy t29.
    pub fn get_backend(&self, p0: Option<&str>, _p1: &str) -> Option<String> {
        self.resolve_app(p0).map(|(_app, backend)| backend)
    }

    /// Resolve `(app_id, backend_url)` for a host. Used by the metrics catalog
    /// so every request event carries the matched app name, not just the URL.
    pub fn resolve_app(&self, p0: Option<&str>) -> Option<(String, String)> {
        let host = p0.and_then(|h| h.split(':').next())?.trim();
        if host.is_empty() {
            return None;
        }
        let data = self.data.read().ok()?;
        for app in &data.apps {
            for h in &app.s47 {
                if h.eq_ignore_ascii_case(host) {
                    return Some((app.s46.clone(), app.s48.clone()));
                }
                // Wildcard: *.example.com matches sub.example.com
                if let Some(suffix) = h.strip_prefix("*.") {
                    if host.len() > suffix.len()
                        && host.ends_with(suffix)
                        && host.as_bytes()[host.len() - suffix.len() - 1] == b'.'
                    {
                        return Some((app.s46.clone(), app.s48.clone()));
                    }
                }
            }
        }
        None
    }

    pub fn hostname_map(&self) -> HashMap<String, String> {
        let data = match self.data.read() {
            Ok(g) => g,
            Err(_) => return HashMap::new(),
        };
        let mut map = HashMap::new();
        for app in &data.apps {
            for h in &app.s47 {
                map.insert(h.clone(), app.s48.clone());
            }
        }
        map
    }

    pub fn list_apps(&self) -> Vec<t30> {
        self.data.read().map(|d| d.apps.clone()).unwrap_or_default()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn register_and_list() {
        let dir = std::env::temp_dir().join(format!("approuter_reg_test_{}", std::process::id()));
        let _ = std::fs::create_dir_all(&dir);
        let reg = t32::new(&dir);
        reg.register(t30 {
            s46: "test-website".into(),
            s47: vec!["test.example.com".into()],
            s48: "http://127.0.0.1:9999".into(),
        })
        .unwrap();
        let apps = reg.list_apps();
        assert_eq!(apps.len(), 1);
        assert_eq!(apps[0].s46, "test-website");
        assert_eq!(apps[0].s47, ["test.example.com"]);
        let _ = std::fs::remove_dir_all(&dir);
    }

    #[test]
    fn register_rejects_empty_backend() {
        let dir = std::env::temp_dir().join(format!("approuter_reg_test2_{}", std::process::id()));
        let _ = std::fs::create_dir_all(&dir);
        let reg = t32::new(&dir);
        let err = reg
            .register(t30 {
                s46: "x".into(),
                s47: vec![],
                s48: "".into(),
            })
            .unwrap_err();
        assert!(err.contains("empty"));
        let _ = std::fs::remove_dir_all(&dir);
    }

    #[test]
    fn wildcard_hostname_matching() {
        let dir = std::env::temp_dir().join(format!("approuter_reg_wildcard_{}", std::process::id()));
        let _ = std::fs::create_dir_all(&dir);
        let reg = t32::new(&dir);
        reg.register(t30 {
            s46: "ronin".into(),
            s47: vec!["ronin-sites.pro".into(), "*.ronin-sites.pro".into()],
            s48: "http://127.0.0.1:8000".into(),
        })
        .unwrap();
        // Exact match
        assert_eq!(reg.get_backend(Some("ronin-sites.pro"), "/"), Some("http://127.0.0.1:8000".into()));
        // Wildcard match
        assert_eq!(reg.get_backend(Some("test.ronin-sites.pro"), "/"), Some("http://127.0.0.1:8000".into()));
        assert_eq!(reg.get_backend(Some("deep.sub.ronin-sites.pro"), "/"), Some("http://127.0.0.1:8000".into()));
        // Should NOT match bare suffix without subdomain
        assert_eq!(reg.get_backend(Some("notronin-sites.pro"), "/"), None);
        let _ = std::fs::remove_dir_all(&dir);
    }

    #[test]
    fn hostname_collision_rejected() {
        let dir = std::env::temp_dir().join(format!("approuter_reg_collision_{}", std::process::id()));
        let _ = std::fs::create_dir_all(&dir);
        let reg = t32::new(&dir);
        reg.register(t30 {
            s46: "app-a".into(),
            s47: vec!["shared.example.com".into()],
            s48: "http://127.0.0.1:3000".into(),
        })
        .unwrap();
        let err = reg
            .register(t30 {
                s46: "app-b".into(),
                s47: vec!["shared.example.com".into()],
                s48: "http://127.0.0.1:4000".into(),
            })
            .unwrap_err();
        assert!(err.starts_with("conflict:"), "expected conflict error, got: {}", err);
        assert!(err.contains("app-a"));
        // Self-update should still work
        reg.register(t30 {
            s46: "app-a".into(),
            s47: vec!["shared.example.com".into(), "new.example.com".into()],
            s48: "http://127.0.0.1:3000".into(),
        })
        .unwrap();
        let _ = std::fs::remove_dir_all(&dir);
    }

    #[test]
    fn hostname_map_and_get_backend() {
        let dir = std::env::temp_dir().join(format!("approuter_reg_test3_{}", std::process::id()));
        let _ = std::fs::create_dir_all(&dir);
        let reg = t32::new(&dir);
        reg.register(t30 {
            s46: "app1".into(),
            s47: vec!["foo.com".into(), "www.foo.com".into()],
            s48: "http://localhost:8000".into(),
        })
        .unwrap();
        let map = reg.hostname_map();
        assert_eq!(map.get("foo.com"), Some(&"http://localhost:8000".to_string()));
        assert_eq!(reg.get_backend(Some("foo.com"), "/"), Some("http://localhost:8000".into()));
        assert_eq!(reg.get_backend(Some("www.foo.com:443"), "/"), Some("http://localhost:8000".into()));
        let _ = std::fs::remove_dir_all(&dir);
    }
}