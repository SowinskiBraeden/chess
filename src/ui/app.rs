use crate::chess::{Color, Game, Piece};
use eframe::egui;

pub struct App {
    game: Game,
    selected: Option<u8>,
}

impl Default for App {
    fn default() -> Self {
        Self {
            game: Game::new(),
            selected: None,
        }
    }
}

fn piece_image(color: Color, piece: Piece) -> egui::ImageSource<'static> {
    match (color, piece) {
        (Color::White, Piece::Pawn) => egui::include_image!("../../assets/pieces/cburnett/wP.svg"),
        (Color::White, Piece::Knight) => {
            egui::include_image!("../../assets/pieces/cburnett/wN.svg")
        }
        (Color::White, Piece::Bishop) => {
            egui::include_image!("../../assets/pieces/cburnett/wB.svg")
        }
        (Color::White, Piece::Rook) => egui::include_image!("../../assets/pieces/cburnett/wR.svg"),
        (Color::White, Piece::Queen) => {
            egui::include_image!("../../assets/pieces/cburnett/wQ.svg")
        }
        (Color::White, Piece::King) => egui::include_image!("../../assets/pieces/cburnett/wK.svg"),
        (Color::Black, Piece::Pawn) => egui::include_image!("../../assets/pieces/cburnett/bP.svg"),
        (Color::Black, Piece::Knight) => {
            egui::include_image!("../../assets/pieces/cburnett/bN.svg")
        }
        (Color::Black, Piece::Bishop) => {
            egui::include_image!("../../assets/pieces/cburnett/bB.svg")
        }
        (Color::Black, Piece::Rook) => egui::include_image!("../../assets/pieces/cburnett/bR.svg"),
        (Color::Black, Piece::Queen) => {
            egui::include_image!("../../assets/pieces/cburnett/bQ.svg")
        }
        (Color::Black, Piece::King) => egui::include_image!("../../assets/pieces/cburnett/bK.svg"),
    }
}

impl App {
    fn handle_click(&mut self, clicked: u8) {
        if let Some(from) = self.selected {
            if from == clicked {
                self.selected = None;
                return;
            }

            let is_legal_move = self.game.board().moves(from) & (1u64 << clicked) != 0;
            if is_legal_move {
                let _ = self.game.play_move(from, clicked, Some(Piece::Queen));
                self.selected = None;
                return;
            }
        }

        let turn = self.game.board().turn;
        self.selected = match self.game.board().square_to_piece(clicked) {
            Some((color, _)) if color == turn => Some(clicked),
            _ => None,
        };
    }

    fn side_panel(&self, ui: &mut egui::Ui) {
        ui.heading("Chess");
        ui.separator();

        let turn = match self.game.board().turn {
            Color::White => "White",
            Color::Black => "Black",
        };
        ui.label(format!("Turn: {turn}"));

        ui.separator();
        ui.label(format!("White score: {}", self.game.score(Color::White)));
        ui.label(format!("Black score: {}", self.game.score(Color::Black)));
    }
}

impl eframe::App for App {
    fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
        egui::Panel::right("side_panel")
            .default_size(180.0)
            .min_size(140.0)
            .show(ui, |ui| self.side_panel(ui));

        let available = ui.available_rect_before_wrap();
        let size = available.size().min_elem(); // ensures its square
        let board_rect = egui::Rect::from_center_size(available.center(), egui::vec2(size, size));
        let response = ui.allocate_rect(board_rect, egui::Sense::click());
        let painter = ui.painter_at(board_rect);
        let square = board_rect.width() / 8.0;

        if response.clicked() {
            if let Some(pos) = response.interact_pointer_pos() {
                let file = ((pos.x - board_rect.min.x) / square) as i32;
                let rank = 7 - ((pos.y - board_rect.min.y) / square) as i32;
                if (0..8).contains(&file) && (0..8).contains(&rank) {
                    self.handle_click((rank * 8 + file) as u8);
                }
            }
        }

        let legal_moves = self.selected.map_or(0, |from| self.game.board().moves(from));

        for rank in 0..8 {
            for file in 0..8 {
                let sq_index = rank * 8 + file;
                let min =
                    board_rect.min + egui::vec2(file as f32 * square, (7 - rank) as f32 * square);
                let rect = egui::Rect::from_min_size(min, egui::vec2(square, square));

                let color = if Some(sq_index) == self.selected {
                    egui::Color32::from_rgb(130, 150, 90) // selected
                } else if legal_moves & (1u64 << sq_index) != 0 {
                    egui::Color32::from_rgb(90, 130, 150) // legal destination
                } else if (rank + file) % 2 == 0 {
                    egui::Color32::from_rgb(70, 70, 70)
                } else {
                    egui::Color32::from_rgb(40, 40, 40)
                };

                painter.rect_filled(rect, 0, color);

                if let Some((piece_color, piece)) =
                    self.game.board().square_to_piece(sq_index as u8)
                {
                    egui::Image::new(piece_image(piece_color, piece)).paint_at(ui, rect);
                }
            }
        }
    }
}
