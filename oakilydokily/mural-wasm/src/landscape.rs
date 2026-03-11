// Copyright (c) 2026 The Cochran Block. All rights reserved.
//! Landscape texture: mural.png background with crisp 8-bit pixels.

use macroquad::prelude::*;
use macroquad::texture::FilterMode;

/// f120=landscape_load. Landscape background from mural.png. None if load fails.
pub async fn load(path: &str) -> Option<Texture2D> {
    match load_texture(path).await {
        Ok(t) => {
            t.set_filter(FilterMode::Nearest);
            Some(t)
        }
        Err(_) => None,
    }
}
