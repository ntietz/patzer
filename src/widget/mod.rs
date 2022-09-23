use chess::{Board, Color, Piece};
use eframe::egui;
use eframe::egui::Widget;
use crate::theme;

pub struct ChessSquare {
    piece_kind: Option<Piece>,
    piece_color: Option<Color>,
    light: bool,
}

impl ChessSquare {
    pub fn new(piece_kind: Option<Piece>, piece_color: Option<Color>, light: bool) -> Self {
        Self {
            piece_kind,
            piece_color,
            light,
        }
    }
}

pub struct ChessBoard {
    board: Board,
    // TODO: last move
}

impl ChessBoard {
    pub fn new(board: Board) -> Self {
        Self {
            board
        }
    }
}

impl Widget for ChessSquare {
    fn ui(self, ui: &mut egui::Ui) -> egui::Response {
        let (id, rect) = ui.allocate_space(egui::vec2(50.0, 50.0));
        let response = ui.interact(rect, id, egui::Sense::click_and_drag());

        let painter = ui.painter();
        let bg_color = if self.light {
            theme::light_square()
        } else {
            theme::dark_square()
        };

        painter.rect_filled(rect, egui::Rounding::none(), bg_color);

        if let (Some(piece), Some(color)) = (self.piece_kind, self.piece_color) {
            let color_symbol = piece_symbol(piece, Color::Black);
            let line_symbol = piece_symbol(piece, Color::White);

            let text_color = match color {
                Color::White => egui::Color32::WHITE,
                Color::Black => egui::Color32::BLACK,
            };

            painter.text(
                rect.center(),
                egui::Align2::CENTER_CENTER,
                color_symbol,
                egui::FontId::proportional(30.0),
                text_color,
            );
            painter.text(
                rect.center(),
                egui::Align2::CENTER_CENTER,
                line_symbol,
                egui::FontId::proportional(30.0),
                egui::Color32::BLACK,
            );
        }

        response
    }
}

impl Widget for ChessBoard {
    fn ui(self, ui: &mut egui::Ui) -> egui::Response {
        let square_min_size = 50.0;

        ui.spacing_mut().item_spacing = egui::vec2(0.0, 0.0);

        let (id, rect) =
            ui.allocate_space(egui::vec2(8.0 * square_min_size, 8.0 * square_min_size));
        let response = ui.interact(rect, id, egui::Sense::click_and_drag());

        let mut board_ui = ui.child_ui(
            rect,
            egui::Layout::left_to_right(eframe::emath::Align::Center),
        );

        for file_idx in 0..8usize {
            let (_id, file_rect) =
                board_ui.allocate_space(egui::vec2(square_min_size, 8.0 * square_min_size));

            let mut file_ui = board_ui.child_ui(
                file_rect,
                egui::Layout::top_down(eframe::emath::Align::Center),
            );

            for rank_idx in (0..8usize).rev() {
                let rank = chess::Rank::from_index(rank_idx);
                let file = chess::File::from_index(file_idx);
                let square = chess::Square::make_square(rank, file);

                let piece = self.board.piece_on(square);
                let color = self.board.color_on(square);
                let light = (rank_idx + file_idx) % 2 == 0;

                file_ui.add(ChessSquare::new(piece, color, light));
            }
        }

        response
    }
}

fn piece_symbol(piece: Piece, color: Color) -> &'static str {
    match (piece, color) {
        (Piece::King, Color::White) => "♔",
        (Piece::Queen, Color::White) => "♕",
        (Piece::Rook, Color::White) => "♖",
        (Piece::Bishop, Color::White) => "♗",
        (Piece::Knight, Color::White) => "♘",
        (Piece::Pawn, Color::White) => "♙",
        (Piece::King, Color::Black) => "♚",
        (Piece::Queen, Color::Black) => "♛",
        (Piece::Rook, Color::Black) => "♜",
        (Piece::Bishop, Color::Black) => "♝",
        (Piece::Knight, Color::Black) => "♞",
        (Piece::Pawn, Color::Black) => "♟",
    }
}
