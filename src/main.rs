use eframe::egui;

use crate::app::AppState;

mod app;

fn main() {
    let mut native_options = eframe::NativeOptions::default();
    native_options.initial_window_size = Some(egui::vec2(800.0, 150.0));
    eframe::run_native("Othni", native_options, Box::new(|cc| Box::new(AppState::new(cc))));
}

