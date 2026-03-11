// Copyright (c) 2026 The Cochran Block. All rights reserved.
//! rogue-runner: f0=main. t88=GameState f105–f112 methods. c90–c95 consts.

#![allow(
    non_camel_case_types,
    non_snake_case,
    non_upper_case_globals,
    dead_code
)]

use macroquad::prelude::*;
use rogue_runner::{f96, t96};

#[cfg(target_arch = "wasm32")]
fn storage_get(key: &str) -> Option<String> {
    let window = web_sys::window()?;
    let storage = window.local_storage().ok().flatten()?;
    storage.get_item(key).ok().flatten()
}

#[cfg(target_arch = "wasm32")]
fn storage_set(key: &str, value: &str) {
    if let Some(storage) = web_sys::window()
        .and_then(|w| w.local_storage().ok())
        .flatten()
    {
        let _ = storage.set_item(key, value);
    }
}

const c90: u32 = 1000;
const c91: f32 = 0.6;
const c92: f32 = -12.0;
const c93: f32 = 32.0;
const c94: f32 = 24.0;
const c95: f32 = 0.85;

fn window_conf() -> Conf {
    Conf {
        window_title: "Rogue Runner".to_owned(),
        high_dpi: true,
        ..Default::default()
    }
}

#[macroquad::main(window_conf)]
async fn main() {
    let mut st = t88::default();
    st.f105();

    loop {
        st.f111();
        st.f112();
        next_frame().await;
    }
}

struct t88 {
    s88: String,
    s89: u32,
    s90: u32,
    s91: f32,
    s92: f32,
    s93: Option<t96>,
    s94: usize,
    s95: u32,
}
impl Default for t88 {
    fn default() -> Self {
        Self {
            s88: "menu".to_string(),
            s89: 1,
            s90: 0,
            s91: 0.0,
            s92: 0.0,
            s93: None,
            s94: 0,
            s95: 1,
        }
    }
}

impl t88 {
    /// f105=load_progress
    fn f105(&mut self) {
        self.s95 = 1;
        #[cfg(not(target_arch = "wasm32"))]
        if let Ok(l) = std::fs::read_to_string("rogue_runner_level.txt") {
            if let Ok(n) = l.trim().parse::<u32>() {
                self.s95 = n.min(c90);
            }
        }
        #[cfg(target_arch = "wasm32")]
        if let Some(s) = storage_get("rogue_runner_level") {
            if let Ok(n) = s.parse::<u32>() {
                self.s95 = n.min(c90);
            }
        }
    }

    /// f106=save_progress
    fn f106(&self) {
        #[cfg(not(target_arch = "wasm32"))]
        let _ = std::fs::write("rogue_runner_level.txt", self.s89.to_string());
        #[cfg(target_arch = "wasm32")]
        storage_set("rogue_runner_level", &self.s89.to_string());
    }

    /// f107=start_game
    fn f107(&mut self) {
        self.s89 = self.s95;
        self.s93 = Some(f96(self.s89));
        self.s94 = 0;
        self.s91 = screen_height() * c95 - c93;
        self.s92 = 0.0;
        self.s88 = "play".to_string();
    }

    /// f108=jump
    fn f108(&mut self) {
        if self.s88 != "play" {
            return;
        }
        let gy = screen_height() * c95 - c93;
        if self.s91 >= gy - 2.0 {
            self.s92 = c92;
        }
    }

    /// f109=game_over
    fn f109(&mut self) {
        self.s88 = "gameover".to_string();
        self.f106();
    }

    /// f110=level_complete
    fn f110(&mut self) {
        self.s89 = (self.s89 + 1).min(c90);
        self.s90 += self.s89;
        self.f106();
        self.s93 = Some(f96(self.s89));
        self.s94 = 0;
    }

    /// f111=update
    fn f111(&mut self) {
        if is_key_pressed(KeyCode::Space)
            || is_key_pressed(KeyCode::Up)
            || is_mouse_button_pressed(MouseButton::Left)
        {
            if self.s88 == "menu" || self.s88 == "gameover" {
                self.f107();
            } else {
                self.f108();
            }
        }

        if self.s88 != "play" {
            return;
        }

        let ld = self.s93.as_mut().unwrap();
        let gy = screen_height() * c95 - c93;

        self.s92 += c91;
        self.s91 += self.s92 * (get_frame_time() * 60.0 / 16.0);
        if self.s91 > gy {
            self.s91 = gy;
            self.s92 = 0.0;
        }

        let px = screen_width() * 0.2;
        let pl = px;
        let pr = px + c94;
        let pt = self.s91;
        let pb = self.s91 + c93;

        let spd = ld.speed * (get_frame_time() * 60.0 / 16.0);
        for (i, o) in ld.obstacles.iter_mut().enumerate() {
            o.x -= spd;
            if o.x + o.w < pl && i == self.s94 {
                self.s94 += 1;
                self.s90 += 10;
            }
            if o.x < pr && o.x + o.w > pl {
                let ot = screen_height() * c95 - o.h;
                let ob = screen_height() * c95;
                if pb > ot && pt < ob {
                    self.f109();
                    return;
                }
            }
        }
        if self.s94 >= ld.obstacles.len() {
            self.f110();
        }
    }

    /// f112=draw
    fn f112(&self) {
        clear_background(BLACK);

        if self.s88 == "menu" || self.s88 == "gameover" {
            let msg = if self.s88 == "gameover" {
                format!("Game Over. Level {}. Space or Up to retry.", self.s89)
            } else if self.s95 > 1 {
                format!(
                    "Resume from Level {}. 1000 levels. Space, Up arrow, or click.",
                    self.s95
                )
            } else {
                "1000 levels. Jump to survive. Space, Up arrow, or click.".to_string()
            };
            draw_text(
                "Rogue Runner",
                screen_width() / 2.0 - 80.0,
                screen_height() / 2.0 - 40.0,
                32.0,
                SKYBLUE,
            );
            draw_text(
                &msg,
                screen_width() / 2.0 - 150.0,
                screen_height() / 2.0,
                20.0,
                GRAY,
            );
            draw_text(
                "Space, Up arrow, or click to Play",
                screen_width() / 2.0 - 140.0,
                screen_height() / 2.0 + 40.0,
                18.0,
                SKYBLUE,
            );
            return;
        }

        let gy = screen_height() * c95;
        draw_rectangle(0.0, gy, screen_width(), screen_height() - gy, DARKGRAY);

        let px = screen_width() * 0.2;
        draw_rectangle(px, self.s91, c94, c93, SKYBLUE);

        if let Some(ref ld) = self.s93 {
            for o in &ld.obstacles {
                if o.x + o.w > 0.0 {
                    draw_rectangle(o.x, gy - o.h, o.w, o.h, ORANGE);
                }
            }
        }

        draw_text(
            &format!("Level {} · {}", self.s89, self.s90),
            12.0,
            28.0,
            20.0,
            SKYBLUE,
        );
    }
}
