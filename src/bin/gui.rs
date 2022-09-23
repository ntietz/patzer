use eframe::egui;

use patzer::widget::ChessBoard;
use patzer::game_state::GameState;

use std::thread;

use chess::MoveGen;
use patzer::strategies::{first_legal_move, random_move};
use chess::Color;

pub fn main() {
    let mut options = eframe::NativeOptions::default();
    options.default_theme = eframe::Theme::Light;
    options.min_window_size = Some(egui::Vec2::new(500.0, 500.0));

    let state = GameState::new("First legal move", "Random move");
    let app = PatzerApp::new(state.clone());

    // TODO: cancel thread when the GUI closes... or just let it die
    let _ai_thread = thread::spawn(|| { ai_game_play(state); });

    eframe::run_native(
        "Patzer Chess",
        options,
        Box::new(|_cc| Box::new(app))
    );
}

fn ai_game_play(state: GameState) {
    let mut finished = false;

    while !finished {
        let mut game = state.game.lock().expect("game mutex failed to lock");
        println!("{}", game.current_position());

        if game.can_declare_draw() {
            game.declare_draw();
            break;
        }

        let mut valid_moves = MoveGen::new_legal(&game.current_position());
        let current_player = game.side_to_move();

        let selected_move = match current_player {
            Color::White => first_legal_move(&mut valid_moves),
            Color::Black => random_move(&mut valid_moves),
        };

        if let Some(selected_move) = selected_move {
            println!("{:?} makes move {}", current_player, selected_move);
            game.make_move(selected_move);
        } else {
            println!("{:?} resigns", current_player);
            game.resign(current_player);
        }

        finished = game.result().is_some();

        let mut board = state.current_position.lock().expect("board mutex failed to lock");
        *board = game.current_position();

        drop(board);

        thread::sleep(std::time::Duration::from_millis(1_000));
    }

}

struct PatzerApp {
    state: GameState,
}

impl PatzerApp {
    pub fn new(state: GameState) -> Self {
        Self {
            state
        }
    }
}

impl eframe::App for PatzerApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        ctx.request_repaint_after(std::time::Duration::from_millis(10));

        egui::CentralPanel::default().show(ctx, |ui| {
            ui.add(egui::Label::new(&self.state.black_name));
            ui.add(ChessBoard::new(self.state.current_position()));
            ui.add(egui::Label::new(&self.state.white_name));
        });
    }

    fn on_exit(&mut self, _gl: Option<&eframe::glow::Context>) {
        println!("Goodbye! I hope you had fun 👋");
    }
}
