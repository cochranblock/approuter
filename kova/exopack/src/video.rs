// Copyright (c) 2026 The Cochran Block. All rights reserved.
//! Screen capture and recording — trait + xcap impl.
//! Use for demo mode: screenshot before/after actions, optional video.

use std::path::Path;

/// Video recorder trait. Implement for platform-specific capture.
pub trait VideoRecorder: Send + Sync {
    /// Start recording.
    fn start(&mut self) -> Result<(), String>;
    /// Stop and save to path. Returns saved file path.
    fn stop(&mut self, out: &Path) -> Result<std::path::PathBuf, String>;
}

/// Screenshot capture. Returns path to saved PNG.
pub fn capture_screenshot(out_dir: &Path, name: &str) -> Result<std::path::PathBuf, String> {
    #[cfg(feature = "video")]
    {
        use std::fs;
        fs::create_dir_all(out_dir).map_err(|e| e.to_string())?;
        let monitors = xcap::Monitor::all().map_err(|e| e.to_string())?;
        let primary = monitors
            .into_iter()
            .find(|m| m.is_primary().unwrap_or(false))
            .ok_or("no primary monitor")?;
        let image = primary.capture_image().map_err(|e| e.to_string())?;
        let safe = name.replace(['|', '\\', ':', '/', ' '], "_");
        let path = out_dir.join(format!("{}.png", safe));
        image.save(&path).map_err(|e| e.to_string())?;
        Ok(path)
    }
    #[cfg(not(feature = "video"))]
    {
        let _ = (out_dir, name);
        Err("video feature not enabled".into())
    }
}

/// No-op recorder. Use when platform impl not available.
pub struct NoopRecorder;

impl VideoRecorder for NoopRecorder {
    fn start(&mut self) -> Result<(), String> {
        Ok(())
    }
    fn stop(&mut self, _out: &Path) -> Result<std::path::PathBuf, String> {
        Err("video recording not implemented on this platform".into())
    }
}

/// Create a recorder for the current platform.
/// Video encoding (xcap frames → file) deferred; screenshot capture available via capture_screenshot.
pub fn create_recorder() -> Box<dyn VideoRecorder> {
    Box::new(NoopRecorder)
}
