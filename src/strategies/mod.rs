use chess::{ChessMove, MoveGen};

pub fn first_legal_move(moves: &mut MoveGen) -> Option<ChessMove> {
    moves.next()
}

pub fn random_move(moves: &mut MoveGen) -> Option<ChessMove> {
    // TODO: pick a random move!
    None
}
