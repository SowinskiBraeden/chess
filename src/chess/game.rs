use super::{Board, Color};

pub enum GameStatus {
    InProgress,
    Checkmate(Color),
    Stalemate,
    Draw,
}

pub struct Game {
    board: Board,
    history: Vec<Board>,
    score: [u8; 2], // [color] -> score,
    status: GameStatus,
}

impl Game {
    pub fn new() -> Self {
        let board = Board::new();
        Game {
            board: board,
            history: vec![board],
            score: [0u8; 2],
            status: GameStatus::InProgress,
        }
    }

    pub fn play_move(&mut self, from: u8, to: u8) -> bool {
        if !self.board.play_move(from, to) {
            return false;
        }
        self.history.push(self.board); // will copy
        true
    }
}
