use crate::evaluation::{evaluate, Score};
use chess::{Board, ChessMove, Color, MoveGen};
use std::ops::Neg;

/// Basic implementation of alpha-beta pruning.
/// More detail is available on the [CPW Alpha-Beta
/// page](https://www.chessprogramming.org/Alpha-Beta).
///
/// This basic implementation will eventually be supplanted by an implementation
/// which includes transposition tables, iterative deepening, and other
/// enhancements. The interface **is expected to change**.
///
/// Other improvements to come:
///  - Principal variation search, to seed the next round of search
///  - Quiescence search, to avoid the horizon effect
pub fn alpha_beta(board: &Board) -> Option<ChessMove> {
    let moves = MoveGen::new_legal(board);

    let mut best_score = -40_000;
    let mut best_move = None;

    for m in moves {
        let board = board.make_move_new(m);
        let sign = match board.side_to_move() {
            Color::White => -1,
            Color::Black => 1,
        };
        let score = -alpha_beta_helper(board, -80_000, 80_000, -sign, 5);

        if score > best_score {
            best_score = score;
            best_move = Some(m);
        }
        println!("move: {}, score: {}", m, score);
    }

    best_move
}

fn alpha_beta_helper(
    board: Board,
    mut alpha: Score,
    beta: Score,
    sign: Score,
    depth_left: u8,
) -> Score {
    if depth_left == 0 {
        let color = board.side_to_move();
        return evaluate(&board, color, color);
    }

    let moves = MoveGen::new_legal(&board);

    for m in moves {
        let board = board.make_move_new(m);

        let score = -alpha_beta_helper(board, -beta, -alpha, sign.neg(), depth_left - 1);

        if score >= beta {
            return beta;
        }
        if score > alpha {
            alpha = score;
        }
    }

    alpha
}

#[cfg(test)]
mod tests {
    use super::*;
    use chess::Board;
    use std::str::FromStr;

    #[test]
    fn ab_detects_smothered_mate() {
        let board = Board::from_str("2r4k/6pp/8/4N3/8/1Q6/B5PP/7K w - - 0 1").unwrap();
        let candidate = alpha_beta(&board);

        let expected = vec![
            ChessMove::from_san(&board, "Qg8").unwrap(),
            ChessMove::from_san(&board, "Ng6").unwrap(),
        ];

        assert!(candidate.is_some());
        if let Some(candidate) = candidate {
            println!("{:#?}", expected);
            println!("{}", candidate);
            assert!(expected.contains(&candidate));
        }
    }
}
