use eframe::egui;

use patzer::app_state::AppState;
use patzer::widget::ChessBoard;

pub fn main() {
    let options = eframe::NativeOptions {
        default_theme: eframe::Theme::Light,
        min_window_size: Some(egui::Vec2::new(500.0, 500.0)),
        ..Default::default()
    };

    let mut state = AppState::new();
    let app = PatzerApp::new(state.clone());
    state.start_computer_players();

    eframe::run_native("Patzer Chess", options, Box::new(|_cc| Box::new(app)));
}

struct PatzerApp {
    state: AppState,
}

impl PatzerApp {
    pub fn new(state: AppState) -> Self {
        Self { state }
    }
}

impl eframe::App for PatzerApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        ctx.request_repaint_after(std::time::Duration::from_millis(10));

        egui::CentralPanel::default().show(ctx, |ui| {
            let (white_name, black_name) = self.state.player_names();
            let current_position = self.state.current_position();
            ui.add(egui::Label::new(black_name));
            ui.add(ChessBoard::new(current_position));
            ui.add(egui::Label::new(white_name));
        });
    }

    fn on_exit(&mut self, _gl: Option<&eframe::glow::Context>) {
        println!("Goodbye! I hope you had fun 👋");
    }
}
