use crate::game_state::GameState;
use crate::player::{MoveFunction, Player};
use crate::strategies::random_move;
use crate::ui_state::UiState;
use crate::widget::SelectionFn;
use chess::{Board, ChessMove, Color, File, Game, Rank, Square};
use std::sync::{Arc, Mutex};
use std::thread;
use std::thread::JoinHandle;

#[derive(Clone)]
pub struct AppState {
    game_state: Arc<Mutex<GameState>>,
    ui_state: Arc<Mutex<UiState>>,

    white: Arc<Mutex<Player>>,
    black: Arc<Mutex<Player>>,

    white_handle: Arc<Mutex<Option<JoinHandle<()>>>>,
    black_handle: Arc<Mutex<Option<JoinHandle<()>>>>,
}

impl AppState {
    pub fn new() -> Self {
        let white = Player::Human("Me".into());
        //let white = Player::Computer("Random moves".into(), Arc::new(Box::new(random_move)));
        let black = Player::Computer("Random moves".into(), Arc::new(Box::new(random_move)));

        AppState {
            game_state: Arc::new(Mutex::new(GameState::new(white.name(), black.name()))),
            ui_state: Arc::new(Mutex::new(UiState::default())),
            white: Arc::new(Mutex::new(white)),
            black: Arc::new(Mutex::new(black)),
            white_handle: Arc::new(Mutex::new(None)),
            black_handle: Arc::new(Mutex::new(None)),
        }
    }

    pub fn game(&self) -> Game {
        self.game_state.lock().unwrap().game.clone()
    }

    pub fn player_names(&self) -> (String, String) {
        let white = self.white.lock().unwrap().name();
        let black = self.black.lock().unwrap().name();
        (white, black)
    }

    pub fn declare_draw(&self, color: Color) -> bool {
        let game_state = self.game_state.lock().unwrap();

        if game_state.game.side_to_move() == color {
            self.game_state.lock().unwrap().game.declare_draw()
        } else {
            // TODO: log/trace when this happens
            false
        }
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

    pub fn start_computer_players(&mut self) {
        let white_player = self.white.lock().unwrap().clone();
        let state = self.clone();
        let color = Color::White;

        *self.white_handle.lock().unwrap() = Some(std::thread::spawn(move || {
            let move_fn = match white_player.move_function() {
                Some(f) => f,
                None => return,
            };

            run_computer(state, move_fn, color);
        }));

        let black_player = self.black.lock().unwrap().clone();
        let state = self.clone();
        let color = Color::Black;

        *self.black_handle.lock().unwrap() = Some(std::thread::spawn(move || {
            let move_fn = match black_player.move_function() {
                Some(f) => f,
                None => return,
            };

            run_computer(state, move_fn, color);
        }));
    }

    pub fn ui_select_fn(&self) -> Box<SelectionFn> {
        let mut app_state = self.clone();

        let f = move |selection: Option<(usize, usize)>| {
            app_state.ui_set_selected_square(selection);
            println!("s: {:?}", selection);
        };

        Box::new(f)
    }

    pub fn ui_attempt_move_fn(&self) -> Box<SelectionFn> {
        let mut app_state = self.clone();

        let f = move |to_selection: Option<(usize, usize)>| {
            if app_state.human_to_move() {
                let candidate_move = match (
                    app_state.ui_state.lock().unwrap().selected_square,
                    to_selection,
                ) {
                    (Some((r1, f1)), Some((r2, f2))) => {
                        let from_sq =
                            Square::make_square(Rank::from_index(r1), File::from_index(f1));
                        let to_sq = Square::make_square(Rank::from_index(r2), File::from_index(f2));
                        // TODO: handle promotion
                        Some(ChessMove::new(from_sq, to_sq, None))
                    }
                    _ => None,
                };

                if let Some(candidate_move) = candidate_move {
                    println!("cm: {:?}", candidate_move);
                    app_state.make_move(candidate_move);
                }
            }

            app_state.ui_set_selected_square(None);
        };

        Box::new(f)
    }

    pub fn ui_selected_square(&self) -> Option<(usize, usize)> {
        self.ui_state.lock().unwrap().selected_square
    }

    pub fn ui_set_selected_square(&mut self, selection: Option<(usize, usize)>) {
        self.ui_state.lock().unwrap().selected_square = selection;
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
        if game.result().is_some() {
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
            thread::sleep(std::time::Duration::from_millis(1_000));
            app_state.make_move(m);
        } else {
            app_state.resign(color);
        }
    }
}
