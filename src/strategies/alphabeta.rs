use chess::{ChessMove, Game};

/// Basic implementation of alpha-beta pruning.
/// More detail is available on the [CPW Alpha-Beta
/// page](https://www.chessprogramming.org/Alpha-Beta).
///
/// This basic implementation will eventually be supplanted by an implementation
/// which includes transposition tables, iterative deepening, and other
/// enhancements. The interface **is expected to change**.
pub fn alpha_beta(_game: &Game) -> Option<ChessMove> {
    todo!("alpha-beta is not yet implemented")
}
