// ref: https://chessprogramming.org/Bitboards

use std::{char, fmt};
mod bitboard_constants;

#[derive(Clone, Copy)]
#[repr(usize)]
enum Color {
    White,
    Black,
}

#[derive(Clone, Copy)]
#[repr(usize)]
enum Piece {
    Pawn,
    Knight,
    Bishop,
    Rook,
    Queen,
    King,
}

// impl Piece {
//     fn value(&self) -> u8 {
//         match self {
//             Piece::Pawn => 1,
//             Piece::Knight | Piece::Bishop => 3,
//             Piece::Rook => 5,
//             Piece::Queen => 9,
//             _ => 10,
//         }
//     }
// }

struct Board {
    pieces: [[u64; 6]; 2], // [color][piece] -> u64 bitboard
}

impl Board {
    fn new() -> Self {
        let mut board = Board {
            pieces: [[0u64; 6]; 2],
        };

        let white = Color::White as usize;
        let black = Color::Black as usize;

        board.pieces[white][Piece::Pawn as usize] = bitboard_constants::WHITE_PAWN_INIT;
        board.pieces[white][Piece::Knight as usize] = bitboard_constants::WHITE_KNIGHT_INIT;
        board.pieces[white][Piece::Bishop as usize] = bitboard_constants::WHITE_BISHOP_INIT;
        board.pieces[white][Piece::Rook as usize] = bitboard_constants::WHITE_ROOK_INIT;
        board.pieces[white][Piece::Queen as usize] = bitboard_constants::WHITE_QUEEN_INIT;
        board.pieces[white][Piece::King as usize] = bitboard_constants::WHITE_KING_INIT;

        board.pieces[black][Piece::Pawn as usize] = bitboard_constants::BLACK_PAWN_INIT;
        board.pieces[black][Piece::Knight as usize] = bitboard_constants::BLACK_KNIGHT_INIT;
        board.pieces[black][Piece::Bishop as usize] = bitboard_constants::BLACK_BISHOP_INIT;
        board.pieces[black][Piece::Rook as usize] = bitboard_constants::BLACK_ROOK_INIT;
        board.pieces[black][Piece::Queen as usize] = bitboard_constants::BLACK_QUEEN_INIT;
        board.pieces[black][Piece::King as usize] = bitboard_constants::BLACK_KING_INIT;

        board
    }
}

impl fmt::Display for Board {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // Matches the order of Color and Piece enums.
        const PIECE_SYMBOLS: [[char; 6]; 2] = [
            ['P', 'N', 'B', 'R', 'Q', 'K'],
            ['p', 'n', 'b', 'r', 'q', 'k'],
        ];

        for rank in (0..8).rev() {
            write!(f, "{} ", rank + 1)?;

            for file in 0..8 {
                let square = rank * 8 + file;
                let mask = 1u64 << square;
                let mut symbol = '.';

                for (color, pieces) in self.pieces.iter().enumerate() {
                    for (piece, bitboard) in pieces.iter().enumerate() {
                        if bitboard & mask != 0 {
                            symbol = PIECE_SYMBOLS[color][piece];
                        }
                    }
                }

                write!(f, " {symbol}")?;
            }

            writeln!(f)?;
        }

        writeln!(f, "   a b c d e f g h")
    }
}

fn main() {
    let game = Board::new();
    println!("{game}");
}
