use chess::{Board, BoardStatus, Color, MoveGen, Piece};

use super::types::Score;

pub fn material_count(board: Board) -> Score {
    let score = material_for_color(&board, Color::White) - material_for_color(&board, Color::Black);
    score.clamp(-1.0 * CHECKMATE_VALUE, CHECKMATE_VALUE)
}

const PAWN_VALUE: f32 = 1.0;
const BISHOP_VALUE: f32 = 3.0;
const KNIGHT_VALUE: f32 = 3.0;
const ROOK_VALUE: f32 = 5.0;
const QUEEN_VALUE: f32 = 9.0;
const CHECKMATE_VALUE: f32 = 200.0;

fn material_for_color(board: &Board, color: Color) -> Score {
    let color_bitboard = board.color_combined(color);

    let pawn_bitboard = board.pieces(Piece::Pawn) & color_bitboard;
    let bishop_bitboard = board.pieces(Piece::Bishop) & color_bitboard;
    let knight_bitboard = board.pieces(Piece::Knight) & color_bitboard;
    let rook_bitboard = board.pieces(Piece::Rook) & color_bitboard;
    let queen_bitboard = board.pieces(Piece::Queen) & color_bitboard;

    let checkmate = if fast_status(board) == BoardStatus::Checkmate && board.side_to_move() != color
    {
        1.0
    } else {
        0.0
    };

    if board.status() == BoardStatus::Stalemate {
        return 0.0;
    }

    pawn_bitboard.popcnt() as f32 * PAWN_VALUE
        + bishop_bitboard.popcnt() as f32 * BISHOP_VALUE
        + knight_bitboard.popcnt() as f32 * KNIGHT_VALUE
        + rook_bitboard.popcnt() as f32 * ROOK_VALUE
        + queen_bitboard.popcnt() as f32 * QUEEN_VALUE
        + checkmate * CHECKMATE_VALUE
}

/// An optimized version of `board.status()`.
fn fast_status(board: &Board) -> BoardStatus {
    let mut moves = MoveGen::new_legal(board);

    if moves.next().is_some() {
        BoardStatus::Ongoing
    } else if board.checkers() == &chess::EMPTY {
        BoardStatus::Stalemate
    } else {
        BoardStatus::Checkmate
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use chess::Board;
    use std::str::FromStr;

    #[test]
    fn starting_position_is_even() {
        let board =
            Board::from_str("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1").unwrap();
        assert_eq!(material_count(board), 0.0);
    }

    #[test]
    fn recognizes_checkmate() {
        let board =
            Board::from_str("r3r1k1/pbq2pQ1/7p/1pp5/4n3/2B4P/PPP2PP1/R3R1K1 b - - 0 20").unwrap();
        assert_eq!(material_count(board), 198.0);
    }

    #[test]
    fn recognizes_material_count() {
        let board = Board::from_str("4rk1b/1ppb1p2/p1Bp4/8/5q2/7P/P5P1/4R2K b - - 0 27").unwrap();
        assert_eq!(material_count(board), -14.0);
    }
}
