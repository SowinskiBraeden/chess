use super::bitboard;
use super::board::MoveError;
use super::{Board, Color, Piece};

pub enum GameStatus {
    InProgress,
    Checkmate(Color),
    Stalemate,
    Draw,
}

pub struct Game {
    pub board: Board,
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

    pub fn play_move(&mut self, from: u8, to: u8, promote: Option<Piece>) -> Result<(), MoveError> {
        let color = self.board.turn;
        let captured = self.board.play_move(from, to, promote)?;

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

    pub fn checkmate(&self) -> Option<Color> {
        let color = self.board.turn;
        if self.board.king_under_attack(color) && !self.board.has_legal_moves() {
            return Some(color);
        }
        None
    }

    // King not under attack, but no legal moves
    pub fn stalemate(&self) -> bool {
        !self.board.king_under_attack(self.board.turn) && !self.board.has_legal_moves()
    }

    pub fn reset(&mut self) {
        *self = Self::new();
    }
}
