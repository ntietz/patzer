use eframe::{egui, App};
use egui::{menu, Ui};
use std::sync::Arc;

use crate::app_state::AppState;
use crate::strategies::{alpha_beta, first_legal_move, hope_chess, random_move};
use crate::widget::ChessBoard;

use crate::player::Player;

pub struct PatzerApp {
    state: AppState,
}

impl PatzerApp {
    pub fn new(state: AppState) -> Self {
        Self { state }
    }
}

impl App for PatzerApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // repainting has to happen even if we're not interacting with the UI, since the computer
        // might make a move. this could be made more efficient by triggering an explicit repaint
        // *if* there has been a move on the board.
        ctx.request_repaint_after(std::time::Duration::from_millis(10));

        display_main_window(ctx, &mut self.state);
    }

    fn on_exit(&mut self, _gl: Option<&eframe::glow::Context>) {
        println!("Goodbye! I hope you had fun! 👋");
    }
}

fn display_main_window(ctx: &egui::Context, state: &mut AppState) {
    egui::TopBottomPanel::top("menu_panel").show(ctx, |ui| {
        display_menu(ui, state);
    });

    egui::TopBottomPanel::bottom("player_info_panel").show(ctx, |ui| {
        ui.heading(state.status_message());
    });

    egui::CentralPanel::default().show(ctx, |ui| {
        let (white_name, black_name) = state.player_names();
        let current_position = state.current_position();
        let selected_square = state.ui_selected_square();
        let inner_state = state.clone();

        egui::Frame::none().show(ui, |ui| {
            ui.group(|ui| {
                ui.add(egui::Label::new(black_name));
                ui.add(ChessBoard::new(
                    current_position,
                    selected_square,
                    inner_state,
                ));
                ui.add(egui::Label::new(white_name));
            });
        });

        if ui.add(egui::Button::new("resign")).clicked() && state.human_to_move() {
            state.resign(state.side_to_move());
        }

        if ui.add(egui::Button::new("declare draw")).clicked() {
            println!("attempting to declare draw");
            state.declare_draw(state.side_to_move());
        }
    });
}

fn display_menu(ui: &mut Ui, state: &mut AppState) {
    menu::bar(ui, |ui| {
        ui.menu_button("Game", |ui| {
            if ui.button("New game").clicked() {
                state.reset_game();
            }
            // TODO: disable button if already started?
            if ui.button("Start").clicked() {
                state.start_game();
            }
        });

        ui.menu_button("Players", |ui| {
            ui.set_enabled(!state.is_started());

            ui.menu_button("White", |ui| {
                if ui.button("Human").clicked() {
                    state.set_white_player(Player::Human("Human".to_string()));
                } else if ui.button("Random move").clicked() {
                    state.set_white_player(Player::Computer(
                        "Random move".into(),
                        Arc::new(Box::new(random_move)),
                    ));
                } else if ui.button("First legal move").clicked() {
                    state.set_white_player(Player::Computer(
                        "First legal move".into(),
                        Arc::new(Box::new(first_legal_move)),
                    ));
                } else if ui.button("Hope").clicked() {
                    state.set_white_player(Player::Computer(
                        "Hope chess".into(),
                        Arc::new(Box::new(hope_chess)),
                    ));
                } else if ui.button("Negamax alpha-beta").clicked() {
                    state.set_white_player(Player::Computer(
                        "Negamax alpha-beta".into(),
                        Arc::new(Box::new(|g| alpha_beta(&g.current_position()))),
                    ));
                }
            });

            ui.menu_button("Black", |ui| {
                if ui.button("Human").clicked() {
                    state.set_black_player(Player::Human("Human".to_string()));
                } else if ui.button("Random move").clicked() {
                    state.set_black_player(Player::Computer(
                        "Random move".into(),
                        Arc::new(Box::new(random_move)),
                    ));
                } else if ui.button("First legal move").clicked() {
                    state.set_black_player(Player::Computer(
                        "First legal move".into(),
                        Arc::new(Box::new(first_legal_move)),
                    ));
                } else if ui.button("Hope").clicked() {
                    state.set_black_player(Player::Computer(
                        "Hope chess".into(),
                        Arc::new(Box::new(hope_chess)),
                    ));
                } else if ui.button("Negamax alpha-beta").clicked() {
                    state.set_black_player(Player::Computer(
                        "Negamax alpha-beta".into(),
                        Arc::new(Box::new(|g| alpha_beta(&g.current_position()))),
                    ));
                }
            });
        });
    });
}
