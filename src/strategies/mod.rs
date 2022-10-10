use std::collections::{HashSet, HashMap};
use chess::{Board, ChessMove, Color, Game, MoveGen};
use rand::seq::IteratorRandom;
use crate::evaluation::{material_count, Score};


pub fn first_legal_move(game: &Game) -> Option<ChessMove> {
    let mut moves = MoveGen::new_legal(&game.current_position());
    moves.next()
}

pub fn random_move(game: &Game) -> Option<ChessMove> {
    let moves = MoveGen::new_legal(&game.current_position());
    let mut rng = rand::thread_rng();
    moves.choose(&mut rng)
}

/// A simple strategy that ignores that our opponent can make moves and goes for
/// anything we do that can get us more material.
///
/// This is obviously not a great strategy, but mirrors what some humans do and
/// could be interesting to explore as a heuristic for pruning trees or
/// expanding them at the edges.
pub fn greedy(game: &Game) -> Option<ChessMove> {
    let sign = if game.side_to_move() == Color::White { 1.0 } else { 1.0 };
    let (score, m) = greedy_helper(game.current_position(), sign, 4);
    println!("Score: {}", score);
    m
}

fn greedy_helper_iterative(initial_pos: Board, sign: f32, depth: u8) -> (Score, Option<ChessMove>) {
    let moves = MoveGen::new_legal(&initial_pos);

    let max = -1.0 * sign * 200.0;

    let boards: HashSet<Board> = HashSet::new();

    for depth in (0..depth).rev() {
    }

    (max, None)
}

fn greedy_helper(board: Board, sign: f32, depth: u8) -> (Score, Option<ChessMove>) {
    if depth == 0 {
        return (material_count(board) * sign, None);
    }

    let moves = MoveGen::new_legal(&board);
    moves.map(|m| -> (Score, Option<ChessMove>) {
        let board1 = board.make_move_new(m);
        let board2 = null_move_or_random(board1);

        match board2 {
            None => (material_count(board1) * sign, Some(m)),
            Some(board) => (depth as f32 *10.0 + greedy_helper(board, sign, depth-1).0, Some(m))
        }
    }).max_by(|a, b| a.0.partial_cmp(&b.0).unwrap()).unwrap_or((-1.0*sign*200.0, None))
}

fn null_move_or_random(board: Board) -> Option<Board> {
    match board.null_move() {
        Some(b) => Some(b),
        None => {
            let mut moves = MoveGen::new_legal(&board);
            match moves.next() {
                Some(m) => Some(board.make_move_new(m)),
                None => None,
            }
        }
    }
}

pub fn minimax(_game: &Game) -> Option<ChessMove> {
    todo!("implement minimax")
}


#[cfg(test)]
mod tests {
    use super::*;
    use chess::Square;

    #[test]
    fn greedy_plays_e4_on_depth_4() {
        // since greedy looks for what gets it the highest score and ignores
        // opponent moves, if you let it look at a high enough depth it should
        // find Scholar's Mate. so the first move should be 1. e4.

        let game = chess::Game::new();
        let candidate = greedy(&game);

        let expected = ChessMove::new(Square::E2, Square::E4, None);

        assert!(candidate.is_some());
        for candidate in candidate {
            assert_eq!(expected, candidate);
        }
    }
}
