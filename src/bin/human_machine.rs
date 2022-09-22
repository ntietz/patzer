use chess::{Color, Game, MoveGen};

use patzer::prompt::input_human_move;
use patzer::strategies::{first_legal_move, random_move};

pub fn main() {
    let mut game = Game::new();

    while game.result().is_none() {
        println!("{}", game.current_position());

        if game.can_declare_draw() {
            game.declare_draw();
            break;
        }

        let mut valid_moves = MoveGen::new_legal(&game.current_position());
        let current_player = game.side_to_move();

        let selected_move = match current_player {
            Color::White => input_human_move(&game.current_position(), &mut valid_moves),
            Color::Black => random_move(&mut valid_moves),
        };

        if let Some(selected_move) = selected_move {
            println!("{:?} makes move {}", current_player, selected_move);
            game.make_move(selected_move);
        } else {
            println!("{:?} resigns", current_player);
            game.resign(game.side_to_move());
        }
    }

    println!(
        "Game is over! {:?}, {:?}",
        game.result(),
        game.current_position().status()
    );
}
