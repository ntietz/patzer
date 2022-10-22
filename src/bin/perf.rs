pub fn main() {
    let game = chess::Game::new();
    patzer::strategies::alpha_beta(&game.current_position(), 8);
}
