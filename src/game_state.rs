use chess::{Board, Game};
use std::sync::{Arc, Mutex};

#[derive(Clone)]
pub struct GameState {
    pub white_name: String,
    pub black_name: String,

    pub game: Arc<Mutex<Game>>,
    pub current_position: Arc<Mutex<Board>>,
}

impl GameState {
    pub fn new(white_name: &str, black_name: &str) -> Self {
        let game = chess::Game::new();
        let pos = game.current_position();

        Self {
            white_name: white_name.to_string(),
            black_name: black_name.to_string(),

            game: Arc::new(Mutex::new(game)),
            current_position: Arc::new(Mutex::new(pos)),
        }
    }

    pub fn current_position(&self) -> Board {
        *self.current_position.lock().expect("position mutex failed to lock")
    }
}
