// ref: https://chessprogramming.org/Bitboards

use super::bitboard;
use super::bitboard::bitboard_to_squares;
use super::movegen;
use super::piece::{Color, Piece};
use std::{char, fmt};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MoveError {
    InvalidSquare,
    EmptySquare,
    WrongTurn,
    IllegalMove,
}

#[derive(Clone, Copy)]
pub struct Board {
    pub pieces: [[u64; 6]; 2], // [color][piece] -> u64 bitboard
    pub turn: Color,
    castling_rights: u8,
    en_passant: Option<u8>,
    halfmove_clock: u16,
}

impl Board {
    pub fn new() -> Self {
        let mut board = Board {
            pieces: [[0u64; 6]; 2],
            turn: Color::White,
            castling_rights: bitboard::castle_rights::ALL,
            en_passant: None,
            halfmove_clock: 0,
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

    fn occupancy(&self, color: Color) -> u64 {
        self.pieces[color as usize]
            .iter()
            .fold(0, |occupied, bitboard| occupied | bitboard)
    }

    fn all_occupancy(&self) -> u64 {
        self.occupancy(Color::White) | self.occupancy(Color::Black)
    }

    // pub for debug purposes
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

    fn handle_castle_rights(&mut self, color: Color, piece: Piece, from: u8) {
        use bitboard::castle_rights;

        let (queen_side, king_side, queen_rook, king_rook) = match color {
            Color::White => (
                castle_rights::WHITE_QUEEN_SIDE,
                castle_rights::WHITE_KING_SIDE,
                bitboard::WHITE_QUEEN_SIDE_ROOK,
                bitboard::WHITE_KING_SIDE_ROOK,
            ),
            Color::Black => (
                castle_rights::BLACK_QUEEN_SIDE,
                castle_rights::BLACK_KING_SIDE,
                bitboard::BLACK_QUEEN_SIDE_ROOK,
                bitboard::BLACK_KING_SIDE_ROOK,
            ),
        };

        let revoke = match piece {
            Piece::King => king_side | queen_side,
            Piece::Rook if from == queen_rook => queen_side,
            Piece::Rook if from == king_rook => king_side,
            _ => 0,
        };

        self.castling_rights &= !revoke;
    }

    // Move piece from square to square, remove enemy if captured
    // unchecked: doesnt determine if valid move, assumes to/from is valid for a given piece
    fn apply_move_unchecked(
        &mut self,
        color: Color,
        piece: Piece,
        from: u8,
        to: u8,
    ) -> Option<Piece> {
        let to_bitboard = 1u64 << to;
        let mut captured: Option<Piece> = None;

        // remove enemy if captured
        if self.occupancy(color.opposite()) & to_bitboard != 0 {
            let (ec, ep) = match self.square_to_piece(to) {
                Some(r) => r,
                None => panic!("apply_move_unchecked: cant find enemy piece/color"),
            };
            self.set_bitboard(ec, ep, self.pieces[ec as usize][ep as usize] & !to_bitboard);
            captured = Some(ep);
            self.handle_castle_rights(ec, ep, from);
        }

        // move piece to destination
        self.set_bitboard(
            color,
            piece,
            self.pieces[color as usize][piece as usize] & !(1u64 << from) | to_bitboard,
        );

        self.handle_castle_rights(color, piece, from);

        captured
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

    // check if current turn as at least 1 legal move somewhere
    pub fn has_legal_moves(&self) -> bool {
        let occupied = self.occupancy(self.turn);
        for square in bitboard::bitboard_to_squares(occupied) {
            if self.moves(square) != 0 {
                return true;
            }
        }
        false
    }

    pub fn king_under_attack(&self, color: Color) -> bool {
        let enemy = color.opposite();
        let attacks = self.all_attacks(enemy);
        let king = self.pieces[color as usize][Piece::King as usize];
        attacks & king != 0
    }

    pub fn square_to_piece(&self, square: u8) -> Option<(Color, Piece)> {
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

    fn castling_moves(&self, color: Color) -> u64 {
        use bitboard::{castle_files as files, castle_paths, castle_rights, castle_safe};

        let occupied = self.all_occupancy();
        let attacks = self.all_attacks(color.opposite());
        let rooks = self.pieces[color as usize][Piece::Rook as usize];
        let king = self.pieces[color as usize][Piece::King as usize];

        let (king_side, queen_side, king_path, queen_path, rank_offset) = match color {
            Color::White => (
                castle_rights::WHITE_KING_SIDE,
                castle_rights::WHITE_QUEEN_SIDE,
                castle_paths::WHITE_KING_SIDE,
                castle_paths::WHITE_QUEEN_SIDE,
                0,
            ),
            Color::Black => (
                castle_rights::BLACK_KING_SIDE,
                castle_rights::BLACK_QUEEN_SIDE,
                castle_paths::BLACK_KING_SIDE,
                castle_paths::BLACK_QUEEN_SIDE,
                56,
            ),
        };

        if king != 1u64 << (rank_offset + files::KING_START) {
            return 0;
        }

        let mut moves = 0u64;

        // Kingside
        if self.castling_rights & king_side != 0
            && rooks & (1u64 << (rank_offset + files::KING_SIDE_ROOK)) != 0
            && occupied & king_path == 0
            && attacks & (castle_safe::KING_SIDE << rank_offset) == 0
        {
            moves |= 1u64 << (rank_offset + files::KING_SIDE_DEST);
        }

        // Queenside
        if self.castling_rights & queen_side != 0
            && rooks & (1u64 << (rank_offset + files::QUEEN_SIDE_ROOK)) != 0
            && occupied & queen_path == 0
            && attacks & (castle_safe::QUEEN_SIDE << rank_offset) == 0
        {
            moves |= 1u64 << (rank_offset + files::QUEEN_SIDE_DEST);
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
            Piece::King => {
                (movegen::king_attacks(square) & !self.occupancy(color))
                    | self.castling_moves(color)
            }
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

    pub fn play_move(&mut self, from: u8, to: u8) -> Result<Option<Piece>, MoveError> {
        if from >= 64 || to >= 64 {
            return Err(MoveError::InvalidSquare);
        }

        let (color, piece) = self.square_to_piece(from).ok_or(MoveError::EmptySquare)?;

        if color != self.turn {
            return Err(MoveError::WrongTurn);
        }

        if self.moves(from) & (1u64 << to) == 0 {
            return Err(MoveError::IllegalMove);
        }

        let captured_piece = self.apply_move_unchecked(color, piece, from, to);
        self.turn = self.turn.opposite();

        Ok(captured_piece)
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
