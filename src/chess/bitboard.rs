// u64 bit board:
// 0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000
// FILES:                                               <- ... HGFE_DCBA|HGFE_DCBA
// RANKS:                                               <- ... | RANK 2|_| RANK 1|

#[rustfmt::skip]
mod init_positions {
    // A1 = bit 0; H8 = bit 63.
    // Binary comments show each occupied rank from H to A.

    //          swap then flip ->                         rank/file: ABCD EFGH
    pub const WHITE_PAWN_INIT:   u64 = 0x0000_0000_0000_FF00; // r2: 1111 1111
    pub const WHITE_KNIGHT_INIT: u64 = 0x0000_0000_0000_0042; // r1: 0100 0010
    pub const WHITE_BISHOP_INIT: u64 = 0x0000_0000_0000_0024; // r1: 0010 0100
    pub const WHITE_ROOK_INIT:   u64 = 0x0000_0000_0000_0081; // r1: 1000 0001
    pub const WHITE_QUEEN_INIT:  u64 = 0x0000_0000_0000_0008; // r1: 0001 0000
    pub const WHITE_KING_INIT:   u64 = 0x0000_0000_0000_0010; // r1: 0000 1000

    //          swap then flip ->                         rank/file: ABCD EFGH
    pub const BLACK_PAWN_INIT:   u64 = 0x00FF_0000_0000_0000; // r7: 1111 1111
    pub const BLACK_KNIGHT_INIT: u64 = 0x4200_0000_0000_0000; // r8: 0100 0010
    pub const BLACK_BISHOP_INIT: u64 = 0x2400_0000_0000_0000; // r8: 0010 0100
    pub const BLACK_ROOK_INIT:   u64 = 0x8100_0000_0000_0000; // r8: 1000 0001
    pub const BLACK_QUEEN_INIT:  u64 = 0x0800_0000_0000_0000; // r8: 0001 0000
    pub const BLACK_KING_INIT:   u64 = 0x1000_0000_0000_0000; // r8: 0000 1000

    // Starting squares of rookks for castling
    pub const WHITE_QUEEN_SIDE_ROOK: u8 = 0;
    pub const WHITE_KING_SIDE_ROOK:  u8 = 7;
    pub const BLACK_QUEEN_SIDE_ROOK: u8 = 56;
    pub const BLACK_KING_SIDE_ROOK:  u8 = 63;
}

#[rustfmt::skip]
pub mod castle_rights {
    pub const WHITE_KING_SIDE:  u8 = 0b0001;
    pub const WHITE_QUEEN_SIDE: u8 = 0b0010;
    pub const BLACK_KING_SIDE:  u8 = 0b0100;
    pub const BLACK_QUEEN_SIDE: u8 = 0b1000;

    pub const WHITE: u8 = WHITE_KING_SIDE | WHITE_QUEEN_SIDE;
    pub const BLACK: u8 = BLACK_KING_SIDE | BLACK_QUEEN_SIDE;
    pub const ALL:   u8 = WHITE | BLACK;
}

#[rustfmt::skip]
pub mod castle_paths {
    // Squares that must be empty.
    pub const WHITE_KING_SIDE:  u64 = 0x0000_0000_0000_0060;
    pub const WHITE_QUEEN_SIDE: u64 = 0x0000_0000_0000_000E;
    pub const BLACK_KING_SIDE:  u64 = 0x6000_0000_0000_0000;
    pub const BLACK_QUEEN_SIDE: u64 = 0x0E00_0000_0000_0000;
}

#[rustfmt::skip]
pub mod castle_files {
    pub const KING_START:      u8 = 4; // E

    pub const KING_SIDE_DEST:  u8 = 6; // G
    pub const QUEEN_SIDE_DEST: u8 = 2; // C

    pub const KING_SIDE_ROOK:  u8 = 7; // H
    pub const QUEEN_SIDE_ROOK: u8 = 0; // A

    pub const KING_SIDE_ROOK_DEST:  u8 = 5; // F
    pub const QUEEN_SIDE_ROOK_DEST: u8 = 3; // D
}

pub mod castle_safe {
    // Includes the kings starting square.
    pub const KING_SIDE: u64 = 0x70; // E, F, G
    pub const QUEEN_SIDE: u64 = 0x1C; // C, D, E
}

pub use init_positions::*;

// Iterate only over set bits
pub fn bitboard_to_squares(mut board: u64) -> Vec<u8> {
    let mut squares = Vec::new();
    while board != 0 {
        squares.push(board.trailing_zeros() as u8);
        board &= board - 1;
    }
    squares
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
