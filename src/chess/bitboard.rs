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
