use chess::{ChessMove, Game, MoveGen};
use rand::seq::IteratorRandom;

mod alphabeta;

pub use alphabeta::alpha_beta;

pub fn first_legal_move(game: &Game) -> Option<ChessMove> {
    let mut moves = MoveGen::new_legal(&game.current_position());
    moves.next()
}

pub fn random_move(game: &Game) -> Option<ChessMove> {
    let moves = MoveGen::new_legal(&game.current_position());
    let mut rng = rand::thread_rng();
    moves.choose(&mut rng)
}
