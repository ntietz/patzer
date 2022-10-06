use chess::{ChessMove, Game};
use std::sync::Arc;

pub type MoveFunction = dyn Fn(&Game) -> Option<ChessMove> + Send + Sync + 'static;

#[derive(Clone)]
pub enum Player {
    Unset,
    Human(String),
    Computer(String, Arc<Box<MoveFunction>>),
}

impl Player {
    pub fn name(&self) -> String {
        match self {
            Player::Unset => "(none)".into(),
            Player::Human(name) => name.clone(),
            Player::Computer(name, _) => name.clone(),
        }
    }

    pub fn move_function(&self) -> Option<Arc<Box<MoveFunction>>> {
        match self {
            Player::Computer(_, f) => Some(f.clone()),
            _ => None,
        }
    }
}
