// Copyright (c) 2026 The Cochran Block. All rights reserved.
//! Kova autopilot — type prompts into Cursor's agent composer. No API costs.
//! f121=autopilot_run

use std::process::Command;
use std::thread;
use std::time::Duration;

fn focus_cursor() {
    for pattern in ["Cursor", "cursor", "Cursor -"] {
        let out = Command::new("xdotool")
            .args(["search", "--name", pattern, "windowactivate"])
            .output();
        if let Ok(o) = out {
            if o.status.success() {
                eprintln!("[autopilot] Focused Cursor");
                return;
            }
        }
    }
    eprintln!("[autopilot] Could not focus Cursor (xdotool?). Ensure Cursor has focus.");
}

/// f121=autopilot_run. Run autopilot: focus Cursor, open composer, type prompt, submit.
pub fn run(prompt: String) -> anyhow::Result<()> {
    focus_cursor();
    thread::sleep(Duration::from_millis(500));

    let mut enigo = enigo::Enigo::new(&enigo::Settings::default())
        .map_err(|e| anyhow::anyhow!("enigo init: {}", e))?;

    use enigo::{Direction, Key, Keyboard};

    #[cfg(target_os = "macos")]
    {
        enigo.key(Key::Meta, Direction::Press).map_err(|e| anyhow::anyhow!("enigo: {}", e))?;
        enigo.key(Key::Unicode('i'), Direction::Click).map_err(|e| anyhow::anyhow!("enigo: {}", e))?;
        enigo.key(Key::Meta, Direction::Release).map_err(|e| anyhow::anyhow!("enigo: {}", e))?;
    }
    #[cfg(not(target_os = "macos"))]
    {
        enigo.key(Key::Control, Direction::Press).map_err(|e| anyhow::anyhow!("enigo: {}", e))?;
        enigo.key(Key::Unicode('i'), Direction::Click).map_err(|e| anyhow::anyhow!("enigo: {}", e))?;
        enigo.key(Key::Control, Direction::Release).map_err(|e| anyhow::anyhow!("enigo: {}", e))?;
    }
    thread::sleep(Duration::from_millis(800));

    enigo.text(&prompt).map_err(|e| anyhow::anyhow!("enigo text: {}", e))?;
    thread::sleep(Duration::from_millis(200));

    enigo.key(Key::Return, Direction::Click).map_err(|e| anyhow::anyhow!("enigo: {}", e))?;

    eprintln!("[autopilot] Submitted {} chars to Cursor composer", prompt.len());
    Ok(())
}
