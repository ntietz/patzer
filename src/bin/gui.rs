use eframe::egui;

use patzer::app_state::AppState;
use patzer::windows::PatzerApp;

pub fn main() {
    let options = eframe::NativeOptions {
        default_theme: eframe::Theme::Light,
        min_window_size: Some(egui::Vec2::new(500.0, 500.0)),
        ..Default::default()
    };

    let state = AppState::new();
    let app = PatzerApp::new(state);

    eframe::run_native("Patzer Chess", options, Box::new(|_cc| Box::new(app)));
}
