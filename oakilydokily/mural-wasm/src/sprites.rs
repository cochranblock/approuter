// Copyright (c) 2026 The Cochran Block. All rights reserved.
//! SpriteSheet: algorithmic grid slicing. TextureAtlas: species + animation mapping.

use macroquad::prelude::*;
use macroquad::texture::FilterMode;
use serde::{Deserialize, Serialize};

/// t119=Species.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Species {
    Cat,
    Dog,
    GuineaPig,
}

/// t120=Animation.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Animation {
    Walk,
    Interaction,
    Sleeping,
}

/// t121=SpriteSheet. Algorithmic slicing of sprite sheet by uniform grid.
#[derive(Debug)]
pub struct SpriteSheet {
    pub texture: Texture2D,
    pub cols: u32,
    pub rows: u32,
    pub cell_w: f32,
    pub cell_h: f32,
}

impl SpriteSheet {
    /// f121=SpriteSheet::load. Grid slice; fallback 128x96 placeholder.
    pub async fn load(path: &str) -> Self {
        let texture = match load_texture(path).await {
            Ok(t) => t,
            Err(_) => {
                let img = Image::gen_image_color(128, 96, WHITE);
                let t = Texture2D::from_image(&img);
                t.set_filter(FilterMode::Nearest);
                return SpriteSheet {
                    texture: t,
                    cols: 4,
                    rows: 3,
                    cell_w: 32.,
                    cell_h: 32.,
                };
            }
        };
        texture.set_filter(FilterMode::Nearest);
        let (w, h) = (texture.width(), texture.height());
        let cols = 4;
        let rows = 3;
        SpriteSheet {
            texture,
            cols,
            rows,
            cell_w: w / cols as f32,
            cell_h: h / rows as f32,
        }
    }

    /// f123=SpriteSheet::cell_rect. Returns pixel Rect for DrawTextureParams source.
    pub fn cell_rect(&self, col: u32, row: u32) -> Rect {
        let x = col as f32 * self.cell_w;
        let y = row as f32 * self.cell_h;
        Rect::new(x, y, self.cell_w, self.cell_h)
    }
}

/// t122=TextureAtlas. Cats row 0, Dogs row 1, Guinea Pigs row 2. Cols: Walk(0), Interaction(1), Sleeping(2), Kiss(3).
#[derive(Debug)]
pub struct TextureAtlas<'a> {
    sheet: &'a SpriteSheet,
}

impl<'a> TextureAtlas<'a> {
    /// f122=TextureAtlas::from_sheet.
    pub fn from_sheet(sheet: &'a SpriteSheet) -> Self {
        TextureAtlas { sheet }
    }

    /// f124=TextureAtlas::frame. Species row, anim col.
    pub fn frame(&self, species: Species, anim: Animation, frame: u32) -> Rect {
        let row = match species {
            Species::Cat => 0,
            Species::Dog => 1,
            Species::GuineaPig => 2,
        };
        let col = match anim {
            Animation::Walk => 0,
            Animation::Interaction => 1,
            Animation::Sleeping => 2,
        };
        let col = (col + frame) % self.sheet.cols;
        self.sheet.cell_rect(col, row)
    }

    /// f125=TextureAtlas::kiss_frame. Guinea Pig col 3.
    pub fn kiss_frame(&self, frame: u32) -> Rect {
        self.sheet.cell_rect(3.min(frame), 2)
    }

    /// f126=TextureAtlas::texture.
    pub fn texture(&self) -> &Texture2D {
        &self.sheet.texture
    }
}
