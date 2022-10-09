pub type Coords = (usize, usize);

#[derive(Clone, Default)]
pub struct UiState {
    pub selected_square: Option<Coords>,
}
