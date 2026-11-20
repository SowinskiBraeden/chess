// ref: https://chessprogramming.org/Bitboards

use super::bitboard;
use super::bitboard::bitboard_to_squares;
use super::movegen;
use super::piece::{Color, Piece};
use std::{char, fmt};

#[derive(Clone, Copy)]
pub struct Board {
    pub pieces: [[u64; 6]; 2], // [color][piece] -> u64 bitboard
    turn: Color,
}

impl Board {
    pub fn new() -> Self {
        let mut board = Board {
            pieces: [[0u64; 6]; 2],
            turn: Color::White,
        };

        let white = Color::White as usize;
        let black = Color::Black as usize;

        board.pieces[white][Piece::Pawn as usize] = bitboard::WHITE_PAWN_INIT;
        board.pieces[white][Piece::Knight as usize] = bitboard::WHITE_KNIGHT_INIT;
        board.pieces[white][Piece::Bishop as usize] = bitboard::WHITE_BISHOP_INIT;
        board.pieces[white][Piece::Rook as usize] = bitboard::WHITE_ROOK_INIT;
        board.pieces[white][Piece::Queen as usize] = bitboard::WHITE_QUEEN_INIT;
        board.pieces[white][Piece::King as usize] = bitboard::WHITE_KING_INIT;

        board.pieces[black][Piece::Pawn as usize] = bitboard::BLACK_PAWN_INIT;
        board.pieces[black][Piece::Knight as usize] = bitboard::BLACK_KNIGHT_INIT;
        board.pieces[black][Piece::Bishop as usize] = bitboard::BLACK_BISHOP_INIT;
        board.pieces[black][Piece::Rook as usize] = bitboard::BLACK_ROOK_INIT;
        board.pieces[black][Piece::Queen as usize] = bitboard::BLACK_QUEEN_INIT;
        board.pieces[black][Piece::King as usize] = bitboard::BLACK_KING_INIT;

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
            let squares = bitboard_to_squares(*bitboard);
            let piece = Piece::from(piece);
            for square in squares {
                attacks |= movegen::attacks(color, piece, square, occupied);
            }
        }

        attacks
    }

    // Move piece from square to square, remove enemy if captured
    // unchecked: doesnt determine if valid move, assumes to/from is valid for a given piece
    fn apply_move_unchecked(&mut self, color: Color, piece: Piece, from: u8, to: u8) {
        let to_bitboard = 1u64 << to;

        // remove enemy if captured
        if self.occupancy(color.opposite()) & to_bitboard != 0 {
            let (ec, ep) = match self.square_to_piece(to) {
                Some(r) => r,
                None => panic!("apply_move_unchecked: cant find enemy piece/color"),
            };
            self.set_bitboard(ec, ep, self.pieces[ec as usize][ep as usize] & !to_bitboard);
            // self.score[color as usize] += ep.value();
        }

        // move piece to destination
        self.set_bitboard(
            color,
            piece,
            self.pieces[color as usize][piece as usize] & !(1u64 << from) | to_bitboard,
        );
    }

    /*
     * checks a bitboard of psuedo-legal moves (possible moves without
     * checking king safety) removes illegal moves (putting king in check),
     * or saving king from check
     */
    fn filter_legal_moves(&self, moves: &mut u64, starting_square: u8, color: Color, piece: Piece) {
        let squares = bitboard_to_squares(*moves);
        for dest in squares {
            // copy current state, and apply moves/captures for each possible dest in squares
            let mut next_board = *self;

            next_board.apply_move_unchecked(color, piece, starting_square, dest);

            // filter out moves that leave king under attack
            if next_board.king_under_attack(color) {
                *moves &= !(1u64 << dest);
            }
        }
    }

    fn king_under_attack(&self, color: Color) -> bool {
        let enemy = color.opposite();
        let attacks = self.all_attacks(enemy);
        let king = self.pieces[color as usize][Piece::King as usize];
        attacks & king != 0
    }

    fn square_to_piece(&self, square: u8) -> Option<(Color, Piece)> {
        assert!(square < 64, "invalid square index");
        let bit = 1u64 << square;
        for color in 0..2 {
            for piece in 0..6 {
                if self.pieces[color][piece] & bit != 0 {
                    return Some((Color::from(color), Piece::from(piece)));
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

    // legal moves for a given square
    pub fn moves(&self, square: u8) -> u64 {
        assert!(square < 64, "invalid square index");

        let (color, piece) = match self.square_to_piece(square) {
            Some(r) => r,
            None => return 0u64,
        };

        if self.turn != color {
            return 0u64;
        }

        // generate pseudo-legal moves only
        let mut moves = match piece {
            Piece::Pawn => self.pawn_moves(color, square),
            _ => {
                // every other piece move is simple and can be determined here
                movegen::attacks(color, piece, square, self.all_occupancy())
                    & !self.occupancy(color)
            }
        };

        // prevent king capture + filter for just legal moves
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
