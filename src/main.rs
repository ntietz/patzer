use chess::{Game, MoveGen};

use patzer::strategies::first_legal_move;


pub fn main() {
    let mut game = Game::new();

    while game.result().is_none() {
        println!("{}", game.current_position());

        if game.can_declare_draw() {
            game.declare_draw();
            break;
        }

        let mut valid_moves = MoveGen::new_legal(&game.current_position());
        let selected_move = first_legal_move(&mut valid_moves);
        if let Some(selected_move) = selected_move {
            game.make_move(selected_move);
        } else {
            game.resign(game.side_to_move());
        }
    }

    println!("Game is over! {:?}, {:?}", game.result(), game.current_position().status());
}
