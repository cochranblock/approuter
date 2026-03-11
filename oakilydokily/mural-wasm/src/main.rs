// Copyright (c) 2026 The Cochran Block. All rights reserved.
//! Interactive 2D mural: pets, proximity detection, scroll-triggered scenes.
//! Build: cargo build --target wasm32-unknown-unknown -p mural-wasm --release

#![allow(dead_code)]

use macroquad::prelude::*;

mod bridge;
mod landscape;
mod sprites;
mod pet;
mod scenes;

use bridge::{get_scroll_x, get_scroll_y};
use sprites::{SpriteSheet, TextureAtlas};
use sprites::Species;
use pet::{Pet, PetState};
use scenes::SceneState;

/// f119=window_conf. Transparent canvas, resizable.
fn window_conf() -> Conf {
    Conf {
        window_title: "OakilyDokily Mural".to_owned(),
        window_resizable: true,
        platform: miniquad::conf::Platform {
            framebuffer_alpha: true, // transparent canvas (clear_background alpha 0)
            ..Default::default()
        },
        ..Default::default()
    }
}

#[macroquad::main(window_conf)]
async fn main() {
    let landscape = landscape::load("/assets/mural.png").await;
    let sprite_sheet = SpriteSheet::load("/assets/1000003453.png").await;
    let atlas = TextureAtlas::from_sheet(&sprite_sheet);

    let mut pets = vec![
        Pet::new(Species::Cat, vec2(100., 200.), &atlas),
        Pet::new(Species::Dog, vec2(250., 180.), &atlas),
        Pet::new(Species::GuineaPig, vec2(400., 220.), &atlas),
    ];

    let mut scene = SceneState::default();
    let mut last_scroll_x: f32 = 0.;
    let mut last_scroll_y: f32 = 0.;

    loop {
        // JS bridge: scrollX, scrollY, mouse set by JS via mural_set_scroll_x/y, mural_set_mouse
        let scroll_x = get_scroll_x();
        let scroll_y = get_scroll_y();
        let mouse_pos = bridge::get_mouse_pos();

        // Scroll-triggered scenes: Cozy Nook at scroll_x, Tubing at scroll_y, Doggy Door at footer
        let was_triggered = scene.doggy_door_triggered;
        scene.update(scroll_x, scroll_y, last_scroll_x, last_scroll_y);
        if scene.doggy_door_triggered && !was_triggered {
            for p in &mut pets {
                p.enter_exodus();
            }
        }
        last_scroll_x = scroll_x;
        last_scroll_y = scroll_y;
        let _ = mouse_pos;

        // Occlusion culling: process pets in viewport, plus Exodus pets until they exit
        let viewport = Rect::new(0., scroll_y, screen_width(), screen_height());
        let visible_pets: Vec<usize> = pets
            .iter()
            .enumerate()
            .filter(|(_, p)| {
                viewport.contains(p.pos)
                    || (p.state == PetState::Exodus && p.pos.x >= -50.)
            })
            .map(|(i, _)| i)
            .collect();

        // Proximity detection: same species within 30px -> Interaction
        for &i in &visible_pets {
            for &j in &visible_pets {
                if i >= j {
                    continue;
                }
                let (pi, pj) = (pets[i].species, pets[j].species);
                if pi == pj && pets[i].pos.distance(pets[j].pos) < 30. {
                    pets[i].state = PetState::Interacting;
                    pets[j].state = PetState::Interacting;
                    if pi == Species::GuineaPig {
                        pets[i].trigger_kiss();
                        pets[j].trigger_kiss();
                    }
                }
            }
        }

        // Update visible pets only
        let dt = get_frame_time();
        for &i in &visible_pets {
            pets[i].update(dt, &atlas);
        }

        // Draw
        if landscape.is_some() {
            clear_background(Color::from_rgba(0, 0, 0, 0)); // transparent over landscape
        } else {
            clear_background(Color::from_rgba(0xe8, 0xee, 0xf2, 255)); // fallback when mural.png fails
        }

        if let Some(ref tex) = landscape {
            draw_texture_ex(
                tex,
                0.,
                0.,
                WHITE,
                DrawTextureParams {
                    dest_size: Some(vec2(screen_width(), screen_height())),
                    ..Default::default()
                },
            );
        }

        scene.draw();

        for &i in &visible_pets {
            pets[i].draw(&atlas);
        }

        next_frame().await;
    }
}

