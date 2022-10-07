use eframe::egui;
use eframe::App;

use crate::app_state::AppState;
use crate::widget::ChessBoard;

pub struct PatzerApp {
    state: AppState,
}

impl PatzerApp {
    pub fn new(state: AppState) -> Self {
        Self { state }
    }
}

impl App for PatzerApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        ctx.request_repaint_after(std::time::Duration::from_millis(10));

        egui::CentralPanel::default().show(ctx, |ui| {
            let (white_name, black_name) = self.state.player_names();
            let current_position = self.state.current_position();
            let selected_square = self.state.ui_selected_square();
            let select_fn = self.state.ui_select_fn();
            let move_fn = self.state.ui_attempt_move_fn();
            ui.add(egui::Label::new(black_name));
            ui.add(ChessBoard::new(
                current_position,
                selected_square,
                select_fn,
                move_fn,
            ));
            ui.add(egui::Label::new(white_name));
        });
    }

    fn on_exit(&mut self, _gl: Option<&eframe::glow::Context>) {
        println!("Goodbye! I hope you had fun! 👋");
    }
}
