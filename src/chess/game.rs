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
