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

impl eframe::App for App {
    fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
        let available = ui.available_rect_before_wrap();
        let size = available.size().min_elem(); // ensures its square
        let board_rect = egui::Rect::from_center_size(available.center(), egui::vec2(size, size));
        let response = ui.allocate_rect(board_rect, egui::Sense::click());
        let painter = ui.painter_at(board_rect);
        let square = board_rect.width() / 8.0;

        for rank in 0..8 {
            for file in 0..8 {
                let sq_index = rank * 8 + file;
                let min =
                    board_rect.min + egui::vec2(file as f32 * square, (7 - rank) as f32 * square);
                let rect = egui::Rect::from_min_size(min, egui::vec2(square, square));

                let color = if Some(sq_index) == self.selected {
                    egui::Color32::from_rgb(130, 150, 90) // selected
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
