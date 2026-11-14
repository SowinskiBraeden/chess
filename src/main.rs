// ref: https://chessprogramming.org/Bitboards

use std::{char, fmt};
mod bitboard_constants;
mod movegen;

#[derive(Clone, Copy)]
#[repr(usize)]
pub enum Color {
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
//             Piece::King => 10,
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

    fn occupancy(&self, color: Color) -> u64 {
        self.pieces[color as usize]
            .iter()
            .fold(0, |occupied, bitboard| occupied | bitboard)
    }

    fn all_occupancy(&self) -> u64 {
        self.occupancy(Color::White) | self.occupancy(Color::Black)
    }
}

impl fmt::Display for Board {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
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

fn debug_bitboard(b: u64, square: u8) {
    for rank in (0..8).rev() {
        print!("{} ", rank + 1);

        for file in 0..8 {
            let s = rank * 8 + file;
            let mask = 1u64 << s;
            let mut symbol = '.';

            if b & mask != 0 {
                symbol = 'x';
            }

            if 1u64 << square & mask != 0 {
                symbol = '@';
            }

            print!(" {symbol}");
        }
        println!();
    }
    println!("   a b c d e f g h")
}

fn main() {
    let game = Board::new();
    println!("{game}");

    println!("\nPawn Debug White");
    debug_bitboard(movegen::pawn_attacks(20, Color::White), 20);

    println!("\nPawn Debug Black");
    debug_bitboard(movegen::pawn_attacks(8, Color::Black), 8);

    println!("\nKnight Debug");
    debug_bitboard(movegen::knight_attacks(1), 1);

    println!("\nKnight Debug");
    debug_bitboard(movegen::knight_attacks(27), 27);

    println!("\nBishop Debug");
    debug_bitboard(movegen::bishop_attacks(27), 27);

    println!("\nRook Debug");
    debug_bitboard(movegen::rook_attacks(26), 26);

    println!("\nQueen Debug");
    debug_bitboard(movegen::queen_attacks(27), 27);

    println!("\nKing Debug");
    debug_bitboard(movegen::king_attacks(27), 27);

    println!("\nKing Debug");
    debug_bitboard(movegen::king_attacks(7), 7);
}
