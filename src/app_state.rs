use crate::game_state::GameState;
use crate::player::{MoveFunction, Player};
use crate::strategies::{first_legal_move, hope_chess};
use crate::ui_state::UiState;
use chess::{Board, ChessMove, Color, File, Game, GameResult, Piece, Rank, Square};
use std::sync::{Arc, Mutex};
use std::thread;
use std::thread::JoinHandle;

#[derive(Clone)]
pub struct AppState {
    game_state: Arc<Mutex<GameState>>,
    ui_state: Arc<Mutex<UiState>>,

    white: Arc<Mutex<Player>>,
    black: Arc<Mutex<Player>>,

    handles: Arc<Mutex<Vec<JoinHandle<()>>>>,
}

impl AppState {
    pub fn new() -> Self {
        let white = Player::Computer("Hope".into(), Arc::new(Box::new(hope_chess)));
        let black = Player::Computer("First legal".into(), Arc::new(Box::new(first_legal_move)));

        AppState {
            game_state: Arc::new(Mutex::new(GameState::new(white.name(), black.name()))),
            ui_state: Arc::new(Mutex::new(UiState::default())),
            white: Arc::new(Mutex::new(white)),
            black: Arc::new(Mutex::new(black)),
            handles: Arc::new(Mutex::new(vec![])),
        }
    }

    pub fn game(&self) -> Game {
        self.game_state.lock().unwrap().game.clone()
    }

    pub fn is_started(&self) -> bool {
        self.game_state.lock().unwrap().started
    }

    pub fn is_finished(&self) -> bool {
        self.game_state.lock().unwrap().game.result().is_some()
    }

    pub fn start_game(&self) {
        self.game_state.lock().unwrap().started = true;
        self.start_computer_players();
    }

    pub fn status_message(&self) -> &'static str {
        let game_state = self.game_state.lock().unwrap();

        if !game_state.started {
            "Not started"
        } else if let Some(result) = game_state.game.result() {
            match result {
                GameResult::WhiteCheckmates => "White wins (checkmate)",
                GameResult::BlackResigns => "White wins (black resigned)",
                GameResult::BlackCheckmates => "Black wins (checkmate)",
                GameResult::WhiteResigns => "Black wins (white resigned)",
                GameResult::Stalemate => "Draw by stalemate",
                GameResult::DrawDeclared => "Draw declared",
                GameResult::DrawAccepted => "Draw by agreement",
            }
        } else {
            "In progress"
        }
    }

    pub fn set_white_player(&self, player: Player) {
        (*self.white.lock().unwrap()) = player;
    }

    pub fn set_black_player(&self, player: Player) {
        (*self.black.lock().unwrap()) = player;
    }

    pub fn player_names(&self) -> (String, String) {
        let white = self.white.lock().unwrap().name();
        let black = self.black.lock().unwrap().name();
        (white, black)
    }

    pub fn declare_draw(&self, color: Color) -> bool {
        let mut game_state = self.game_state.lock().unwrap();

        println!("declare_draw:b");
        if game_state.game.side_to_move() == color {
            game_state.game.declare_draw()
        } else {
            println!("attempted to declare draw when not available");
            false
        }
    }

    pub fn side_to_move(&self) -> Color {
        println!("side_to_move");
        self.game_state.lock().unwrap().game.side_to_move()
    }

    pub fn resign(&self, color: Color) {
        self.game_state.lock().unwrap().game.resign(color);
    }

    pub fn make_move(&self, m: ChessMove) {
        self.game_state.lock().unwrap().game.make_move(m);
    }

    pub fn current_position(&mut self) -> Board {
        self.game_state.lock().unwrap().game.current_position()
    }

    pub fn human_to_move(&self) -> bool {
        match self.game_state.lock().unwrap().game.side_to_move() {
            Color::White => self.white.lock().unwrap().is_human(),
            Color::Black => self.black.lock().unwrap().is_human(),
        }
    }

    pub fn reset_game(&mut self) {
        {
            let mut game_state = self.game_state.lock().unwrap();
            let mut ui_state = self.ui_state.lock().unwrap();

            let white = self.white.lock().unwrap();
            let black = self.black.lock().unwrap();

            *game_state = GameState::new(white.name(), black.name());
            *ui_state = UiState::default();
        } // release locks before we wait for the threads to halt

        for handle in self.handles.lock().unwrap().drain(0..) {
            handle
                .join()
                .expect("Error waiting for computer opponent to conclude");
        }
    }

    fn start_computer_players(&self) {
        let white_player = self.white.lock().unwrap().clone();
        let state = self.clone();
        let color = Color::White;

        let mut handles = self.handles.lock().unwrap();

        handles.push(std::thread::spawn(move || {
            let move_fn = match white_player.move_function() {
                Some(f) => f,
                None => return,
            };

            run_computer(state, move_fn, color);
        }));

        let black_player = self.black.lock().unwrap().clone();
        let state = self.clone();
        let color = Color::Black;

        handles.push(std::thread::spawn(move || {
            let move_fn = match black_player.move_function() {
                Some(f) => f,
                None => return,
            };

            run_computer(state, move_fn, color);
        }));
    }

    pub fn ui_select_square(&self, selection: Option<(usize, usize)>) {
        if self.is_started() && !self.is_finished() {
            self.ui_state.lock().unwrap().selected_square = selection;
        }
    }

    pub fn ui_attempt_move(&self, to_selection: Option<(usize, usize)>, promote_to: Option<Piece>) {
        if !self.is_started() || self.is_finished() {
            return;
        }

        if self.human_to_move() {
            if let (Some((r1, f1)), Some((r2, f2))) =
                (self.ui_state.lock().unwrap().selected_square, to_selection)
            {
                let from_sq = Square::make_square(Rank::from_index(r1), File::from_index(f1));
                let to_sq = Square::make_square(Rank::from_index(r2), File::from_index(f2));
                self.make_move(ChessMove::new(from_sq, to_sq, promote_to));
            };
        }

        self.ui_select_square(None);
    }

    pub fn ui_selected_square(&self) -> Option<(usize, usize)> {
        self.ui_state.lock().unwrap().selected_square
    }
}

impl Default for AppState {
    fn default() -> Self {
        Self::new()
    }
}

fn run_computer(app_state: AppState, f: Arc<Box<MoveFunction>>, color: Color) {
    loop {
        let game = app_state.game();
        if app_state.is_finished() || !app_state.is_started() {
            break;
        }

        if game.side_to_move() != color {
            thread::sleep(std::time::Duration::from_millis(100));
            continue;
        }

        if game.can_declare_draw() {
            app_state.declare_draw(color);
        }

        if let Some(m) = f(&game) {
            thread::sleep(std::time::Duration::from_millis(100));
            app_state.make_move(m);
        } else {
            app_state.resign(color);
        }
    }
}
