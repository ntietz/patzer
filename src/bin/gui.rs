use eframe::egui;

use patzer::widget::ChessBoard;

pub fn main() {
    let mut options = eframe::NativeOptions::default();
    options.default_theme = eframe::Theme::Light;

    eframe::run_native(
        "Patzer Chess",
        options,
        Box::new(|_cc| Box::new(PatzerApp::default())),
    );
}

struct PatzerApp {
    game: chess::Game,
}

impl Default for PatzerApp {
    fn default() -> Self {
        Self {
            game: chess::Game::new(),
        }
    }
}

impl eframe::App for PatzerApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.add(ChessBoard::new(self.game.current_position()));
        });
    }

    fn on_exit(&mut self, _gl: Option<&eframe::glow::Context>) {
        println!("Goodbye! I hope you had fun 👋");
    }
}
