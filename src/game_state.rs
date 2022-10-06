use chess::{Board, Game};

#[derive(Clone)]
pub struct GameState {
    pub white_name: String,
    pub black_name: String,

    pub started: bool,

    pub game: Game,
}

impl GameState {
    pub fn new(white_name: String, black_name: String) -> Self {
        let game = chess::Game::new();

        Self {
            white_name,
            black_name,

            started: false,

            game,
        }
    }

    pub fn current_position(&self) -> Board {
        self.game.current_position()
    }
}
