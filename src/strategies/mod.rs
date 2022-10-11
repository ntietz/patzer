use chess::{ChessMove, Game, MoveGen};
use rand::seq::IteratorRandom;

mod hope;
pub use hope::hope_chess;

pub fn first_legal_move(game: &Game) -> Option<ChessMove> {
    let mut moves = MoveGen::new_legal(&game.current_position());
    moves.next()
}

pub fn random_move(game: &Game) -> Option<ChessMove> {
    let moves = MoveGen::new_legal(&game.current_position());
    let mut rng = rand::thread_rng();
    moves.choose(&mut rng)
}

pub fn minimax(_game: &Game) -> Option<ChessMove> {
    todo!("implement minimax")
}
