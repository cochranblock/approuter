// Copyright (c) 2026 The Cochran Block. All rights reserved.
//! rogue-runner: 1000 levels, procedural, offline. f95=mulberry32 f96=generate_level

#![allow(non_camel_case_types, non_snake_case, dead_code)]

/// f95 = mulberry32 — deterministic PRNG for level generation
pub fn f95(seed: u32) -> impl FnMut() -> f32 {
    let mut s = seed as i32;
    move || {
        s = s.wrapping_add(0x6D2B79F5);
        ((s as u32) as f32) / 4294967296.0
    }
}

/// t95 = Obstacle
#[derive(Clone)]
pub struct t95 {
    pub x: f32,
    pub h: f32,
    pub w: f32,
}

/// t96 = LevelData
#[derive(Clone)]
pub struct t96 {
    pub speed: f32,
    pub obstacles: Vec<t95>,
}

/// f96 = generate_level — deterministic level from seed(level_num)
pub fn f96(level_num: u32) -> t96 {
    let mut rng = f95(level_num);
    let speed = 3.0 + level_num as f32 * 0.08;
    let count = 5 + (level_num / 50) as usize;
    let mut obstacles = Vec::with_capacity(count);
    for i in 0..count {
        obstacles.push(t95 {
            x: 400.0 + (i as f32) * 220.0 + rng() * 120.0,
            h: 30.0 + rng() * 35.0,
            w: 24.0 + rng() * 16.0,
        });
    }
    t96 { speed, obstacles }
}
