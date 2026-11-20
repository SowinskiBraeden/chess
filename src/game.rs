// ref: https://chessprogramming.org/Bitboards

use crate::bitboard_constants;
use crate::movegen;
use std::{char, fmt};

#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(usize)]
pub enum Color {
    White,
    Black,
}

impl Color {
    fn opposite(&self) -> Color {
        match self {
            Color::White => Color::Black,
            Color::Black => Color::White,
        }
    }

    fn from(value: usize) -> Color {
        assert!(value < 2, "invalid color id");
        match value {
            0 => Color::White,
            _ => Color::Black,
        }
    }
}

#[derive(Clone, Copy)]
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
    fn value(&self) -> u8 {
        match self {
            Piece::Pawn => 1,
            Piece::Knight | Piece::Bishop => 3,
            Piece::Rook => 5,
            Piece::Queen => 9,
            Piece::King => 10,
        }
    }

    fn from(value: usize) -> Piece {
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

#[derive(Clone, Copy)]
pub struct Board {
    pub pieces: [[u64; 6]; 2], // [color][piece] -> u64 bitboard
    turn: Color,
    score: [u8; 2], // [color] -> score
}

// Iterate only over set bits
fn board_to_squares(mut board: u64) -> Vec<u8> {
    let mut squares = Vec::new();
    while board != 0 {
        squares.push(board.trailing_zeros() as u8);
        board &= board - 1;
    }
    squares
}

impl Board {
    pub fn new() -> Self {
        let mut board = Board {
            pieces: [[0u64; 6]; 2],
            turn: Color::White,
            score: [0, 0],
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

    pub fn set_bitboard(&mut self, color: Color, piece: Piece, bitboard: u64) {
        self.pieces[color as usize][piece as usize] = bitboard;
    }

    pub fn occupancy(&self, color: Color) -> u64 {
        self.pieces[color as usize]
            .iter()
            .fold(0, |occupied, bitboard| occupied | bitboard)
    }

    pub fn all_occupancy(&self) -> u64 {
        self.occupancy(Color::White) | self.occupancy(Color::Black)
    }

    pub fn all_attacks(&self, color: Color) -> u64 {
        let occupied = self.all_occupancy();
        let mut attacks = 0u64;
        for (piece, bitboard) in self.pieces[color as usize].iter().enumerate() {
            let squares = board_to_squares(*bitboard);
            let piece = Piece::from(piece);
            for square in squares {
                attacks |= movegen::attacks(color, piece, square, occupied);
            }
        }

        attacks
    }

    /*
     * checks a bitboard of psuedo-legal moves (possible moves without
     * checking king safety) removes illegal moves (putting king in check),
     * or saving king from check
     */
    fn filter_legal_moves(&self, moves: &mut u64, starting_square: u8, color: Color, piece: Piece) {
        let enemy = color.opposite();
        let squares = board_to_squares(*moves);
        for s in squares {
            // clone current state, and apply moves/captures for each possible move in squares
            let to = 1u64 << s;
            let mut next_board = *self; // copy board

            // remove enemy if captured
            if next_board.occupancy(enemy) & to != 0 {
                let (ep, ec) = match next_board.square_to_piece(s) {
                    Some(r) => r,
                    None => panic!("filter_legal_moves: cant determine enemy (s) piece/color"),
                };
                next_board.set_bitboard(ec, ep, next_board.pieces[ec as usize][ep as usize] & !to);
            }

            // move piece to possible `to` square destination
            next_board.set_bitboard(
                color,
                piece,
                next_board.pieces[color as usize][piece as usize] & !(1u64 << starting_square) | to,
            );

            // filter out moves that leave king under attack
            if next_board.king_under_attack(color) {
                *moves &= !to;
            }
        }
    }

    fn king_under_attack(&self, color: Color) -> bool {
        let enemy = color.opposite();
        let attacks = self.all_attacks(enemy);
        let king = self.pieces[color as usize][Piece::King as usize];
        attacks & king != 0
    }

    fn square_to_piece(&self, square: u8) -> Option<(Piece, Color)> {
        assert!(square < 64, "invalid square index");
        let bit = 1u64 << square;
        for color in 0..2 {
            for piece in 0..6 {
                if self.pieces[color][piece] & bit != 0 {
                    return Some((Piece::from(piece), Color::from(color)));
                }
            }
        }
        None
    }

    // Handles more complicated pawn movements, one step/two step, captures
    fn pawn_moves(&self, color: Color, square: u8) -> u64 {
        let (enemy, forward, starting_rank): (Color, i16, u8) = match color {
            Color::White => (Color::Black, 8, 1),
            Color::Black => (Color::White, -8, 6),
        };

        let occupied = self.all_occupancy();
        let mut moves = movegen::pawn_attacks(square, color) & self.occupancy(enemy);
        let one_step = i16::from(square) + forward;

        if (0..64).contains(&one_step) && occupied & (1u64 << one_step) == 0 {
            moves |= 1u64 << one_step;

            if square / 8 == starting_rank {
                let two_step = one_step + forward;
                if occupied & (1u64 << two_step) == 0 {
                    moves |= 1u64 << two_step;
                }
            }
        }

        moves
    }

    pub fn moves(&self, square: u8) -> u64 {
        assert!(square < 64, "invalid square index");

        let (piece, color) = match self.square_to_piece(square) {
            Some((p, c)) => (p, c),
            None => return 0u64,
        };

        if self.turn != color {
            return 0u64;
        }

        let mut moves = match piece {
            Piece::Pawn => self.pawn_moves(color, square),
            _ => {
                // every other piece move is simple and can be determined here
                movegen::attacks(color, piece, square, self.all_occupancy())
                    & !self.occupancy(color)
            }
        };

        // prevent king capture + filter psuedo-legal moves into just legal moves
        moves &= !self.pieces[color.opposite() as usize][Piece::King as usize];
        self.filter_legal_moves(&mut moves, square, color, piece);
        moves
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

// Useful for debugging attacks/moves
pub fn debug_bitboard(b: u64, origin: u8) {
    for rank in (0..8).rev() {
        print!("{} ", rank + 1);

        for file in 0..8 {
            let s = rank * 8 + file;
            let mask = 1u64 << s;
            let mut symbol = '.';

            if b & mask != 0 {
                symbol = 'x';
            }

            if 1u64 << origin & mask != 0 {
                symbol = '@';
            }

            print!(" {symbol}");
        }
        println!();
    }
    println!("   a b c d e f g h");
}
