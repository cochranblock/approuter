//! wowasticker: Pure Rust, offline-first mobile app for student behavioral goals.
//! Local AI dictation via Candle Whisper, SQLite persistence, thumb-zone UI.

#![allow(non_snake_case)]

mod ui;

use dioxus::prelude::*;
use wowasticker::ui;

fn main() {
    dioxus::launch(ui::App);
}
