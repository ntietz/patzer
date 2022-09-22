use chess::{ChessMove, MoveGen};
use rand::seq::IteratorRandom;

pub fn first_legal_move(moves: &mut MoveGen) -> Option<ChessMove> {
    moves.next()
}

pub fn random_move(moves: &mut MoveGen) -> Option<ChessMove> {
    let mut rng = rand::thread_rng();
    moves.choose(&mut rng)
}
