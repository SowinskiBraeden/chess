#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(usize)]
pub enum Color {
    White,
    Black,
}

impl Color {
    pub(super) fn opposite(&self) -> Color {
        match self {
            Color::White => Color::Black,
            Color::Black => Color::White,
        }
    }

    pub(super) fn from(value: usize) -> Color {
        assert!(value < 2, "invalid color id");
        match value {
            0 => Color::White,
            _ => Color::Black,
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(usize)]
pub enum Piece {
    Pawn,
    Knight,
    Bishop,
    Rook,
    Queen,
    King,
}

impl Piece {
    pub(super) fn value(&self) -> u8 {
        match self {
            Piece::Pawn => 1,
            Piece::Knight | Piece::Bishop => 3,
            Piece::Rook => 5,
            Piece::Queen => 9,
            Piece::King => 10,
        }
    }

    pub(super) fn from(value: usize) -> Piece {
        assert!(value < 6, "invalid piece id");
        match value {
            0 => Piece::Pawn,
            1 => Piece::Knight,
            2 => Piece::Bishop,
            3 => Piece::Rook,
            4 => Piece::Queen,
            _ => Piece::King,
        }
    }
}
