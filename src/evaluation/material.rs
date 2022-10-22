use chess::{Board, Color, MoveGen, Piece};

use super::types::Score;

const PAWN_VALUE: Score = 100;
const BISHOP_VALUE: Score = 300;
const KNIGHT_VALUE: Score = 300;
const ROOK_VALUE: Score = 500;
const QUEEN_VALUE: Score = 900;
const CHECKMATE_VALUE: Score = 20_000;

pub fn evaluate(board: &Board, color: Color, to_move: Color) -> Score {
    let moves = MoveGen::new_legal(board);
    let num_moves = moves.len();
    if num_moves == 0 {
        if board.checkers().popcnt() == 0 {
            return 0;
        } else if to_move != color {
            return CHECKMATE_VALUE;
        } else {
            return -CHECKMATE_VALUE;
        }
    }

    let other_color = match color {
        Color::White => Color::Black,
        Color::Black => Color::White,
    };

    material(board, color) + mobility(board, color)
        - material(board, other_color)
        - mobility(board, other_color)
}

fn material(board: &Board, color: Color) -> Score {
    let color_bitboard = board.color_combined(color);

    let pawn_bitboard = board.pieces(Piece::Pawn) & color_bitboard;
    let bishop_bitboard = board.pieces(Piece::Bishop) & color_bitboard;
    let knight_bitboard = board.pieces(Piece::Knight) & color_bitboard;
    let rook_bitboard = board.pieces(Piece::Rook) & color_bitboard;
    let queen_bitboard = board.pieces(Piece::Queen) & color_bitboard;

    pawn_bitboard.popcnt() as Score * PAWN_VALUE
        + bishop_bitboard.popcnt() as Score * BISHOP_VALUE
        + knight_bitboard.popcnt() as Score * KNIGHT_VALUE
        + rook_bitboard.popcnt() as Score * ROOK_VALUE
        + queen_bitboard.popcnt() as Score * QUEEN_VALUE
}

fn mobility(board: &Board, color: Color) -> Score {
    let mobility = if color != board.side_to_move() {
        if let Some(b) = board.null_move() {
            MoveGen::new_legal(&b).len()
        } else {
            0
        }
    } else {
        MoveGen::new_legal(board).len()
    };

    mobility as Score * 10
}

#[cfg(test)]
mod tests {
    use super::*;
    use chess::{Board, Color};
    use std::str::FromStr;

    #[test]
    fn starting_position_is_even() {
        let board =
            Board::from_str("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1").unwrap();
        assert_eq!(evaluate(&board, Color::White, Color::White), 0);
    }

    #[test]
    fn recognizes_checkmate() {
        let board =
            Board::from_str("r3r1k1/pbq2pQ1/7p/1pp5/4n3/2B4P/PPP2PP1/R3R1K1 b - - 0 20").unwrap();
        assert_eq!(evaluate(&board, Color::White, board.side_to_move()), 20_000);
        assert_eq!(
            evaluate(&board, Color::Black, board.side_to_move()),
            -20_000
        );
    }

    #[test]
    fn recognizes_material_count() {
        let board = Board::from_str("4rk1b/1ppb1p2/p1Bp4/8/5q2/7P/P5P1/4R2K b - - 0 27").unwrap();
        assert_eq!(evaluate(&board, Color::White, Color::White), -16_70);
    }
}
