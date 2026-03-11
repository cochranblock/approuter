// Copyright (c) 2026 The Cochran Block. All rights reserved.
//! JS/Rust bridge: scrollY, mouse_pos passed from JS into the engine.
//! Uses #[no_mangle] extern "C" for miniquad gl.js compatibility (no wasm-bindgen).

use std::cell::Cell;

thread_local! {
    static SCROLL_X: Cell<f32> = const { Cell::new(0.) };
    static SCROLL_Y: Cell<f32> = const { Cell::new(0.) };
    static MOUSE_X: Cell<f32> = const { Cell::new(0.) };
    static MOUSE_Y: Cell<f32> = const { Cell::new(0.) };
}

/// f127=mural_set_scroll_x. Exported for gl.js; called from JS as wasm_exports.mural_set_scroll_x(x).
#[cfg(target_arch = "wasm32")]
#[no_mangle]
pub extern "C" fn mural_set_scroll_x(x: f32) {
    SCROLL_X.with(|c| c.set(x));
}

/// f128=mural_set_scroll_y.
#[cfg(target_arch = "wasm32")]
#[no_mangle]
pub extern "C" fn mural_set_scroll_y(y: f32) {
    SCROLL_Y.with(|c| c.set(y));
}

#[cfg(target_arch = "wasm32")]
#[no_mangle]
pub extern "C" fn mural_set_mouse(x: f32, y: f32) {
    MOUSE_X.with(|c| c.set(x));
    MOUSE_Y.with(|c| c.set(y));
}


pub fn get_scroll_x() -> f32 {
    SCROLL_X.with(|c| c.get())
}

/// f131=get_scroll_y.
pub fn get_scroll_y() -> f32 {
    SCROLL_Y.with(|c| c.get())
}

/// f132=get_mouse_pos.
pub fn get_mouse_pos() -> (f32, f32) {
    (MOUSE_X.with(|c| c.get()), MOUSE_Y.with(|c| c.get()))
}
