use crate::evaluation::{material_count, Score};
use chess::{Board, ChessMove, Color, Game, MoveGen};
use rand::seq::IteratorRandom;

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
pub fn hope_chess(game: &Game) -> Option<ChessMove> {
    let sign = if game.side_to_move() == Color::White {
        1.0
    } else {
        -1.0
    };
    let (score, _, m) = hope_chess_helper_iterative(game.current_position(), sign, 4);
    println!("Score: {}", score);
    m
}

fn hope_chess_helper_iterative(
    board: Board,
    sign: f32,
    depth: u8,
) -> (Score, u8, Option<ChessMove>) {
    if depth == 0 {
        return (material_count(board) * sign, 0, None);
    }

    let moves = MoveGen::new_legal(&board);
    let mut max = -200.0;
    let mut remaining = depth;
    let mut choice = None;

    for m in moves {
        let board = board.make_move_new(m);
        let (score, d) = match null_move_or_random(board) {
            Some(board) => {
                let (score, d, _) = hope_chess_helper_iterative(board, sign, depth - 1);
                (score, d)
            }
            None => (material_count(board) * sign, depth),
        };
        if (max < 200.0 && score > max) || (score >= 200.0 && d > remaining) {
            max = score;
            remaining = d;
            choice = Some(m);
        }
    }

    (max, remaining, choice)
}

fn null_move_or_random(board: Board) -> Option<Board> {
    match board.null_move() {
        Some(b) => Some(b),
        None => {
            let mut moves = MoveGen::new_legal(&board);
            moves.next().map(|m| board.make_move_new(m))
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
    fn hope_chess_plays_e4_on_depth_4() {
        // since hope_chess looks for what gets it the highest score and ignores
        // opponent moves, if you let it look at a high enough depth it should
        // find Scholar's Mate. so the first move should be 1. e4.

        let game = chess::Game::new();
        let candidate = hope_chess(&game);

        let expected = vec![
            ChessMove::new(Square::E2, Square::E4, None),
            ChessMove::new(Square::E2, Square::E3, None),
        ];

        assert!(candidate.is_some());
        for candidate in candidate {
            assert!(expected.contains(&candidate));
        }
    }

    #[test]
    fn hope_chess_mates_in_1() {
        let mut game = chess::Game::new();
        game.make_move(ChessMove::from_san(&game.current_position(), "e4").unwrap());
        game.make_move(ChessMove::from_san(&game.current_position(), "a6").unwrap());
        game.make_move(ChessMove::from_san(&game.current_position(), "Qh5").unwrap());
        game.make_move(ChessMove::from_san(&game.current_position(), "a5").unwrap());
        game.make_move(ChessMove::from_san(&game.current_position(), "Bc4").unwrap());
        game.make_move(ChessMove::from_san(&game.current_position(), "a4").unwrap());

        let expected = vec![
            ChessMove::from_san(&game.current_position(), "Qxf7").unwrap(),
            ChessMove::from_san(&game.current_position(), "Bxf7").unwrap(),
        ];

        let candidate = hope_chess(&game);

        assert!(candidate.is_some());
        for candidate in candidate {
            println!("{:#?}", expected);
            println!("{}", candidate);
            assert!(expected.contains(&candidate));
        }
    }
}
