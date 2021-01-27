//use patzer::board::Board;
use patzer::uci::{handle_message, parse_command, print_ready, print_uci_info, EngineMode, MsgOut};
use std::io;

fn main() {
    //let board = Board::new();

    let mut engine_mode = EngineMode::Init;

    loop {
        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Couldn't read user input.");

        if let Some(msg_in) = parse_command(&input) {
            let (msg_out, new_mode) = handle_message(msg_in, engine_mode);
            engine_mode = new_mode;

            match msg_out {
                MsgOut::UciOk => print_uci_info(),
                MsgOut::ReadyOk => print_ready(),
            }
        }
        // silently ignore parsing failures for now
        // TODO: better error handling
        else {
            println!("{}", input);
        }
    }
}
