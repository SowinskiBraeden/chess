// ref: https://chessprogramming.org/Bitboards

use crate::bitboard_constants;
use crate::movegen;
use std::{char, fmt};

#[derive(Clone, Copy)]
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

    fn from(value: usize) -> Option<Piece> {
        match value {
            0 => Some(Piece::Pawn),
            1 => Some(Piece::Knight),
            2 => Some(Piece::Bishop),
            3 => Some(Piece::Rook),
            4 => Some(Piece::Queen),
            5 => Some(Piece::King),
            _ => None,
        }
    }
}

#[derive(Clone, Copy)]
pub struct Board {
    pieces: [[u64; 6]; 2], // [color][piece] -> u64 bitboard
    turn: Color,
    score: [u8; 2], // [color] -> score
}

fn board_to_squares(board: u64) -> Vec<u8> {
    let mut squares = Vec::new();
    for i in 0u8..64u8 {
        if (1u64 << i) & board != 0 {
            squares.push(i);
        }
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

    pub fn all_attacks(&self, color: Color, occupied: u64) -> u64 {
        let mut attacks = 0u64;
        for (piece, bitboard) in self.pieces[color as usize].iter().enumerate() {
            let squares = board_to_squares(*bitboard);
            let piece = match Piece::from(piece) {
                Some(x) => x,
                None => return 0u64,
            };
            for square in squares {
                attacks |= movegen::attacks(piece, square, color, occupied);
            }
        }

        attacks
    }

    /*
     * validates a set of moves by checking if a move
     * will put the king in check, removes invalid moves
     */
    fn validate_moves(&self, starting_square: u8, moves: &mut u64, color: Color) {
        let enemy = color.opposite();
        let squares = board_to_squares(*moves);
        for s in squares {
            let next_board = self.all_occupancy() & !(1u64 << starting_square) | 1u64 << s;
            let attacks = self.all_attacks(enemy, next_board);

            if attacks & self.pieces[color as usize][Piece::King as usize] != 0 {
                *moves &= !(1u64 << s);
            }
        }
    }

    fn pawn_moves(&self, square: u8, color: Color) -> u64 {
        assert!(square < 64, "invalid square index");

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
                let two_steps = one_step + forward;
                if occupied & (1u64 << two_steps) == 0 {
                    moves |= 1u64 << two_steps;
                }
            }
        }

        self.validate_moves(square, &mut moves, color);

        moves
    }

    fn knight_moves(&self, square: u8, color: Color) -> u64 {
        assert!(square < 64, "invalid square index");
        let knights = self.pieces[color as usize][Piece::Knight as usize];
        if knights & (1u64 << square) == 0 {
            return 0;
        }
        let mut moves = movegen::knight_attacks(square) & !self.occupancy(color);
        self.validate_moves(square, &mut moves, color);

        moves
    }

    fn bishop_moves(&self, square: u8, color: Color) -> u64 {
        assert!(square < 64, "invalid square index");
        let bishops = self.pieces[color as usize][Piece::Bishop as usize];
        if bishops & (1u64 << square) == 0 {
            return 0;
        }
        let mut moves =
            movegen::bishop_attacks(square, self.all_occupancy()) & !self.occupancy(color);
        self.validate_moves(square, &mut moves, color);

        moves
    }

    fn rook_moves(&self, square: u8, color: Color) -> u64 {
        assert!(square < 64, "invalid square index");
        let rooks = self.pieces[color as usize][Piece::Rook as usize];
        if rooks & (1u64 << square) == 0 {
            return 0;
        }
        let mut moves =
            movegen::rook_attacks(square, self.all_occupancy()) & !self.occupancy(color);
        self.validate_moves(square, &mut moves, color);
        moves
    }

    fn queen_moves(&self, square: u8, color: Color) -> u64 {
        assert!(square < 64, "invalid square index");
        let queens = self.pieces[color as usize][Piece::Queen as usize];
        if queens & (1u64 << square) == 0 {
            return 0;
        }
        let mut moves =
            movegen::queen_attacks(square, self.all_occupancy()) & !self.occupancy(color);
        self.validate_moves(square, &mut moves, color);

        moves
    }

    fn king_moves(&self, square: u8, color: Color) -> u64 {
        assert!(square < 64, "invalid square index");
        let kings = self.pieces[color as usize][Piece::King as usize];
        if kings & (1u64 << square) == 0 {
            return 0;
        }
        let mut moves = movegen::king_attacks(square) & !self.occupancy(color);
        self.validate_moves(square, &mut moves, color);
        moves
    }

    pub fn moves(&self, piece: Piece, square: u8, color: Color) -> u64 {
        match piece {
            Piece::Pawn => self.pawn_moves(square, color),
            Piece::Knight => self.knight_moves(square, color),
            Piece::Bishop => self.bishop_moves(square, color),
            Piece::Rook => self.rook_moves(square, color),
            Piece::Queen => self.queen_moves(square, color),
            Piece::King => self.king_moves(square, color),
        }
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

pub fn debug_bitboard(b: u64, square: u8) {
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
