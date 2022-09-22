use chess::{Board, ChessMove, MoveGen};
use std::io::stdin;

pub fn input_human_move(board: &Board, moves: &mut MoveGen) -> Option<ChessMove> {
    let stdin = stdin();
    print!("input your move: ");

    let mut buffer = String::new();
    stdin.read_line(&mut buffer).expect("reading stdin failed");

    if is_resignation(&buffer) {
        return None;
    } else {
        return match ChessMove::from_san(board, &buffer) {
            Ok(m) => Some(m),
            Err(_) => None,
        };
    }
}

/// is_resignation will parse the input string for a few patterns of resigning,
/// and will return true if the input signals a resignation.
fn is_resignation(input: &str) -> bool {
    match input {
        "resign" | "quit" | "flip board" => true,
        _ => false,
    }
}

fn parse_move(input: &str) -> Option<ChessMove> {
    let parts: Vec<_> = input.split_whitespace().collect();
    if parts.len() < 2 || parts.len() > 3 {
        // TODO: log the error
        return None;
    }

    let (from, to) = (parts[0], parts[1]);
    if from.len() != 2 || to.len() != 2 {
        // TODO: log the error
        return None;
    }

    None
}

fn check_move(candidate: ChessMove, moves: Vec<ChessMove>) -> bool {
    false
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_validates_resignations() {
        let resignations = vec!["resign", "quit", "flip board"];

        let not_resignations = vec!["e4"];

        for s in resignations {
            assert!(is_resignation(s), "should be a resignation. input: {}", s);
        }

        for s in not_resignations {
            assert_eq!(
                is_resignation(s),
                false,
                "should not be a resignation. input: {}",
                s
            );
        }
    }
}
