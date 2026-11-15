// ref: https://chessprogramming.org/Bitboards

mod bitboard_constants;
mod game;
use game::{Board, Color, debug_bitboard};
mod movegen;

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
    debug_bitboard(movegen::bishop_attacks(27, 0), 27);

    println!("\nRook Debug");
    debug_bitboard(movegen::rook_attacks(26, 0), 26);

    println!("\nQueen Debug");
    debug_bitboard(movegen::queen_attacks(27, 0), 27);

    println!("\nKing Debug");
    debug_bitboard(movegen::king_attacks(27), 27);

    println!("\nKing Debug");
    debug_bitboard(movegen::king_attacks(7), 7);
}
