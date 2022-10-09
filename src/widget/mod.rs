use crate::app_state::AppState;
use crate::theme;
use chess::{Board, Color, File, Piece, Rank};
use eframe::egui;
use egui::Widget;

pub struct ChessBoard {
    board: Board,
    selected_square: Option<(usize, usize)>,
    state: AppState,
    // TODO: last move
}

impl ChessBoard {
    pub fn new(board: Board, selected_square: Option<(usize, usize)>, state: AppState) -> Self {
        Self {
            board,
            selected_square,
            state,
        }
    }
}

impl Widget for ChessBoard {
    fn ui(mut self, ui: &mut egui::Ui) -> egui::Response {
        let square_min_size = 100.0;

        ui.spacing_mut().item_spacing = egui::vec2(0.0, 0.0);

        let (rect, response) = ui.allocate_exact_size(
            egui::vec2(8.0 * square_min_size, 8.0 * square_min_size),
            egui::Sense::click_and_drag(),
        );

        let x_range = rect.x_range();
        let y_range = rect.y_range();

        for file_idx in 0..8usize {
            for rank_idx in 0..8usize {
                let selected = Some((rank_idx, file_idx)) == self.selected_square;
                let x_offset = (file_idx as f32) * square_min_size;
                let y_offset = ((7 - rank_idx) as f32) * square_min_size;
                let xr = std::ops::RangeInclusive::new(
                    *x_range.start() + x_offset,
                    x_range.start() + x_offset + square_min_size,
                );
                let yr = std::ops::RangeInclusive::new(
                    *y_range.start() + y_offset,
                    y_range.start() + y_offset + square_min_size,
                );

                let square_rect = egui::Rect::from_x_y_ranges(xr, yr);
                let painter = ui.painter_at(square_rect);

                self.paint_square(
                    ui,
                    (rank_idx, file_idx),
                    square_rect,
                    painter,
                    &response,
                    selected,
                );
            }
        }

        response
    }
}

impl ChessBoard {
    fn paint_square(
        &mut self,
        ui: &mut egui::Ui,
        coords: (usize, usize),
        rect: egui::Rect,
        painter: egui::Painter,
        response: &egui::Response,
        selected: bool,
    ) {
        let (rank_idx, file_idx) = coords;
        let board = self.board;
        let rank = chess::Rank::from_index(rank_idx);
        let file = chess::File::from_index(file_idx);
        let square = chess::Square::make_square(rank, file);

        let piece = board.piece_on(square);
        let color = board.color_on(square);
        let light = (rank_idx + file_idx) % 2 == 1;

        let popup_id =
            ui.make_persistent_id(format!("promotion-dialogue-{}-{}", rank_idx, file_idx));

        egui::popup::popup_below_widget(ui, popup_id, response, |ui| {
            if let Some((rank_from, file_from)) = self.selected_square {
                if attempting_promotion(&board, (rank_from, file_from), (rank_idx, file_idx)) {
                    if ui.button("queen").clicked() {
                        println!("queren");
                        self.state
                            .ui_attempt_move(Some((rank_idx, file_idx)), Some(Piece::Queen));
                        ui.memory().toggle_popup(popup_id);
                    } else if ui.button("rook").clicked() {
                        println!("rook");
                        self.state
                            .ui_attempt_move(Some((rank_idx, file_idx)), Some(Piece::Rook));
                        ui.memory().toggle_popup(popup_id);
                    } else if ui.button("knight").clicked() {
                        println!("knight");
                        self.state
                            .ui_attempt_move(Some((rank_idx, file_idx)), Some(Piece::Knight));
                        ui.memory().toggle_popup(popup_id);
                    } else if ui.button("bishop").clicked() {
                        println!("bish");
                        self.state
                            .ui_attempt_move(Some((rank_idx, file_idx)), Some(Piece::Bishop));
                        ui.memory().toggle_popup(popup_id);
                    } else if ui.button("cancel").clicked() {
                        println!("nope");
                        self.state.ui_select_square(None);
                        ui.memory().toggle_popup(popup_id);
                    }
                }
            }
        });

        if self.square_clicked(response, &rect) {
            self.handle_click(ui, rank_idx, file_idx, &board, popup_id);
        }

        let bg_color = if selected {
            theme::selected_square()
        } else if light {
            theme::light_square()
        } else {
            theme::dark_square()
        };

        painter.rect_filled(rect, egui::Rounding::none(), bg_color);

        if let (Some(piece), Some(color)) = (piece, color) {
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
                egui::FontId::proportional(60.0),
                text_color,
            );
            painter.text(
                rect.center(),
                egui::Align2::CENTER_CENTER,
                line_symbol,
                egui::FontId::proportional(60.0),
                egui::Color32::BLACK,
            );
        }
    }

    fn square_clicked(&self, response: &egui::Response, rect: &egui::Rect) -> bool {
        response.clicked()
            && response
                .interact_pointer_pos()
                .map_or(false, |pos| rect.contains(pos))
    }

    fn handle_click(
        &mut self,
        ui: &egui::Ui,
        rank: usize,
        file: usize,
        board: &Board,
        popup_id: egui::Id,
    ) {
        match self.selected_square {
            None => {
                self.state.ui_select_square(Some((rank, file)));
            }
            Some((r2, f2)) if r2 != rank || f2 != file => {
                if !attempting_promotion(board, (r2, f2), (rank, file)) {
                    self.state.ui_attempt_move(Some((rank, file)), None);
                } else {
                    ui.memory().toggle_popup(popup_id);
                }
            }
            _ => {
                self.state.ui_select_square(None);
            }
        }
    }
}

fn attempting_promotion(board: &Board, from: (usize, usize), to: (usize, usize)) -> bool {
    let rank = Rank::from_index(from.0);
    let file = File::from_index(from.1);

    let square = chess::Square::make_square(rank, file);

    if let Some(Piece::Pawn) = board.piece_on(square) {
        return to.0 == 0 || to.0 == 7;
    }

    false
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
