// Copyright (c) 2026 The Cochran Block. All rights reserved.
//! Kova theme. THEME.md palette + professional layout.

use eframe::egui::{self, FontId, Rounding, Stroke, TextStyle, Visuals};

/// Theme colors from THEME.md.
pub mod colors {
    use eframe::egui::Color32;

    pub const BG: Color32 = Color32::from_rgb(0x0a, 0x0a, 0x0f);
    pub const SURFACE: Color32 = Color32::from_rgb(0x14, 0x14, 0x1f);
    pub const SURFACE_ELEVATED: Color32 = Color32::from_rgb(0x1a, 0x1a, 0x28);
    pub const SURFACE_HOVER: Color32 = Color32::from_rgb(0x1a, 0x2a, 0x35);
    pub const PRIMARY: Color32 = Color32::from_rgb(0x00, 0xd4, 0xff);
    pub const SECONDARY: Color32 = Color32::from_rgb(0xa8, 0x55, 0xf7);
    pub const TERTIARY: Color32 = Color32::from_rgb(0x14, 0xb8, 0xa6);
    pub const TEXT: Color32 = Color32::from_rgb(0xe2, 0xe8, 0xf0);
    pub const MUTED: Color32 = Color32::from_rgb(0x64, 0x74, 0x8b);
}

/// Layout constants.
pub mod layout {
    pub const MARGIN: f32 = 16.0;
    pub const PADDING_SM: f32 = 6.0;
    pub const PADDING_MD: f32 = 12.0;
    pub const PADDING_LG: f32 = 16.0;
    pub const GAP: f32 = 8.0;
    pub const RADIUS: f32 = 8.0;
    pub const RADIUS_SM: f32 = 4.0;
}

/// Apply full theme to context.
pub fn apply(ctx: &egui::Context) {
    let mut style = (*ctx.style()).clone();
    style.visuals = visuals();
    style.spacing = spacing();
    style.text_styles = text_styles();
    ctx.set_style(style);
}

fn visuals() -> Visuals {
    let mut v = Visuals::dark();
    v.window_fill = colors::BG;
    v.panel_fill = colors::SURFACE;
    v.override_text_color = Some(colors::TEXT);
    v.faint_bg_color = colors::SURFACE;
    v.extreme_bg_color = colors::BG;
    v.code_bg_color = colors::SURFACE_ELEVATED;
    v.hyperlink_color = colors::PRIMARY;
    v.warn_fg_color = colors::TERTIARY;
    v.error_fg_color = colors::SECONDARY;
    v.window_rounding = Rounding::same(layout::RADIUS);
    v.window_stroke = Stroke::new(1.0, colors::SURFACE_ELEVATED);
    v.widgets.noninteractive.bg_fill = colors::SURFACE;
    v.widgets.noninteractive.rounding = Rounding::same(layout::RADIUS_SM);
    v.widgets.inactive.bg_fill = colors::SURFACE_ELEVATED;
    v.widgets.inactive.rounding = Rounding::same(layout::RADIUS_SM);
    v.widgets.hovered.bg_fill = colors::SURFACE_HOVER;
    v.widgets.hovered.rounding = Rounding::same(layout::RADIUS_SM);
    v.widgets.active.bg_fill = colors::PRIMARY;
    v.widgets.active.bg_stroke = Stroke::new(1.0, colors::PRIMARY);
    v.widgets.active.rounding = Rounding::same(layout::RADIUS_SM);
    v.selection.bg_fill = colors::PRIMARY;
    v.selection.stroke = Stroke::new(1.0, colors::PRIMARY);
    v.collapsing_header_frame = true;
    v
}

fn spacing() -> egui::style::Spacing {
    egui::style::Spacing {
        item_spacing: egui::vec2(layout::GAP, layout::GAP),
        button_padding: egui::vec2(layout::PADDING_MD, layout::PADDING_SM),
        window_margin: egui::Margin::same(layout::MARGIN),
        ..Default::default()
    }
}

fn text_styles() -> std::collections::BTreeMap<TextStyle, FontId> {
    let mut map = std::collections::BTreeMap::new();
    map.insert(TextStyle::Small, FontId::proportional(12.0));
    map.insert(TextStyle::Body, FontId::proportional(14.0));
    map.insert(TextStyle::Monospace, FontId::monospace(13.0));
    map.insert(TextStyle::Button, FontId::proportional(14.0));
    map.insert(TextStyle::Heading, FontId::proportional(20.0));
    map
}

/// Styled frame for message cards.
pub fn message_frame() -> egui::Frame {
    egui::Frame::default()
        .fill(colors::SURFACE)
        .rounding(Rounding::same(layout::RADIUS))
        .inner_margin(egui::Margin::same(layout::PADDING_MD))
        .stroke(Stroke::new(1.0, colors::SURFACE_ELEVATED))
}

/// Styled frame for input area.
pub fn input_frame() -> egui::Frame {
    egui::Frame::default()
        .fill(colors::SURFACE)
        .rounding(Rounding::same(layout::RADIUS))
        .inner_margin(egui::Margin::same(layout::PADDING_MD))
        .stroke(Stroke::new(1.0, colors::SURFACE_ELEVATED))
}

/// Styled frame for panels (backlog, prompts).
pub fn panel_frame() -> egui::Frame {
    egui::Frame::default()
        .fill(colors::SURFACE)
        .rounding(Rounding::same(layout::RADIUS))
        .inner_margin(egui::Margin::same(layout::PADDING_MD))
        .stroke(Stroke::new(1.0, colors::SURFACE_ELEVATED))
}
