// Kova WASM app
use eframe::egui;

pub struct KovaWebApp {
    pub input: String,
    pub messages: Vec<String>,
    pub show_backlog: bool,
}

impl KovaWebApp {
    pub fn new(_cc: &eframe::CreationContext<'_>) -> Self {
        Self {
            input: String::new(),
            messages: vec!["Kova thin client.".into()],
            show_backlog: false,
        }
    }
}

impl eframe::App for KovaWebApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Kova");
        });
    }
}
