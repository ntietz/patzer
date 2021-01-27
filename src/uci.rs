/// Messages get sent from the engine to the GUI.
///
/// This is a subset of the UCI standard, because we're not going to use things
/// like copy protection or registration.
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MsgOut {
    /// This means to send ID, optional options, then "uciok".
    UciOk,

    ReadyOk,
}

/// Messages received from the GUI.
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MsgIn {
    /// Requests information about the engine.
    Uci,

    /// Checks if the engine is ready
    IsReady,
}

/// What the status of the engine is.
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EngineMode {
    /// The engine has just started and may provide info.
    Init,

    /// The engine is able to handle starting a game
    Ready,

    /// There's currently a move being considered.
    Thinking,
    // TODO: add Pondering, once that's supported
}

pub fn handle_message(msg: MsgIn, mode: EngineMode) -> (MsgOut, EngineMode) {
    match (msg, mode) {
        (MsgIn::Uci, _) => (MsgOut::UciOk, mode),
        (MsgIn::IsReady, EngineMode::Init) => (MsgOut::ReadyOk, EngineMode::Ready),
        (MsgIn::IsReady, _) => (MsgOut::ReadyOk, mode),
    }
}

pub fn parse_command(command: &str) -> Option<MsgIn> {
    let parts: Vec<_> = command.split_ascii_whitespace().collect();

    if parts.is_empty() {
        None
    } else {
        match parts[0] {
            "uci" => Some(MsgIn::Uci),
            "isready" => Some(MsgIn::IsReady),
            _ => None,
        }
    }
}

pub fn print_uci_info() {
    println!("id name Patzer 0.1");
    println!("id author Nicholas Tietz-Sokolsky");
    println!("uciok");
}

pub fn print_ready() {
    println!("readyok");
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn parses_uci_request() {
        assert_eq!(parse_command("uci"), Some(MsgIn::Uci));
    }

    #[test]
    fn parses_ready_request() {
        assert_eq!(parse_command("isready"), Some(MsgIn::IsReady));
    }

    #[test]
    fn parses_none_for_garbage() {
        assert_eq!(parse_command("garbage request"), None);
    }

    // TODO: test more of this, and get a tool for code coverage
}
