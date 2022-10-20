use crate::{
    evaluation::{evaluate, Score},
    transposition::{Evaluation, TranspositionTable},
};
use chess::{Board, ChessMove, MoveGen};
use rand::seq::SliceRandom;

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
pub fn alpha_beta(board: &Board, depth: u8) -> Option<ChessMove> {
    let mut transposition_table = TranspositionTable::new();

    let mut best_score = -40_000;
    let mut best_move = None;

    let mut alpha = -80_000;
    let beta = 80_000;

    for m in current_moves(board) {
        let board = board.make_move_new(m);
        let score = -alpha_beta_helper(board, -beta, -alpha, depth - 1, &mut transposition_table);

        if score > best_score {
            best_score = score;
            best_move = Some(m);
        }

        if score > alpha {
            alpha = score;
        }

        println!(
            "transposition table size/hits/misses: {} / {} / {}",
            transposition_table.len(),
            transposition_table.hits(),
            transposition_table.misses()
        );
        println!(
            "{}",
            transposition_table.misses() as i64 - transposition_table.len() as i64
        );
        println!("move: {}, score: {}", m, score);
    }

    best_move
}

fn alpha_beta_helper(
    board: Board,
    mut alpha: Score,
    mut beta: Score,
    depth_left: u8,
    transposition_table: &mut TranspositionTable,
) -> Score {
    let hash = board.get_hash();

    // Reuse results if they've been computed before
    if let Some(entry) = transposition_table.retrieve(hash) {
        if entry.depth == depth_left {
            let score = match entry.eval {
                Evaluation::Exact(score) => return score,
                Evaluation::Beta(score) => {
                    alpha = alpha.min(score);
                    score
                }
                Evaluation::Alpha(score) => {
                    beta = beta.min(score);
                    score
                }
            };

            if alpha >= beta {
                return score;
            }
        }
    }

    if depth_left == 0 {
        let color = board.side_to_move();
        let score = evaluate(&board, color, color);
        transposition_table.store(hash, depth_left, Evaluation::Exact(score));
        return score;
    }

    for m in current_moves(&board) {
        let board = board.make_move_new(m);

        let score = -alpha_beta_helper(board, -beta, -alpha, depth_left - 1, transposition_table);

        if score >= beta {
            transposition_table.store(hash, depth_left, Evaluation::Beta(beta));
            return beta;
        }
        if score > alpha {
            alpha = score;
        }
    }

    transposition_table.store(hash, depth_left, Evaluation::Alpha(alpha));
    alpha
}

/// Generates the current moves for the board in a heuristically ordered fashion.
///
/// Right now the heuristic is "just shuffle them in place randomly," which
/// is at least moderately better than just ignoring the order entirely.
fn current_moves(board: &Board) -> Vec<ChessMove> {
    let mut rng = &mut rand::thread_rng();
    let mut moves: Vec<ChessMove> = MoveGen::new_legal(board).collect();
    moves.shuffle(&mut rng);
    moves
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
