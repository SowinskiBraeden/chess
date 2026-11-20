use super::board::MoveError;
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

    pub fn play_move(&mut self, from: u8, to: u8) -> Result<(), MoveError> {
        let color = self.board.turn;
        let captured = self.board.play_move(from, to)?;

        if let Some(piece) = captured {
            self.score[color as usize] += piece.value();
        }

        self.history.push(self.board);

        Ok(())
    }

    /*
     * read only
     */
    pub fn board(&self) -> &Board {
        &self.board
    }

    pub fn score(&self, color: Color) -> u8 {
        self.score[color as usize]
    }

    pub fn status(&self) -> &GameStatus {
        &self.status
    }

    pub fn history(&self) -> &[Board] {
        &self.history
    }
}
