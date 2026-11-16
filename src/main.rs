// ref: https://chessprogramming.org/Bitboards

mod bitboard_constants;
mod game;
use game::{Board, Color, Piece, debug_bitboard};
mod movegen;

fn main() {
    let mut game = Board::new();

    game.set_bitboard(Color::Black, Piece::Bishop, 1u64 << 32); // puts enemy bishop in sight of king on queen pawn move
    game.set_bitboard(
        Color::Black,
        Piece::Pawn,
        1u64 << 20 | bitboard_constants::BLACK_PAWN_INIT,
    ); // gives white queen pawn ability to capture also to move

    println!("{game}");

    println!("\nDebug Attacks");
    debug_bitboard(game.all_attacks(Color::Black, game.all_occupancy()), 32);

    println!("\nDebug Pawn White");
    debug_bitboard(game.moves(Piece::Pawn, 11, Color::White), 11);

    // println!("\nPawn Debug White");
    // debug_bitboard(movegen::pawn_attacks(20, Color::White), 20);

    // println!("\nPawn Debug Black");
    // debug_bitboard(movegen::pawn_attacks(8, Color::Black), 8);

    // println!("\nKnight Debug");
    // debug_bitboard(movegen::knight_attacks(1), 1);

    // println!("\nKnight Debug");
    // debug_bitboard(movegen::knight_attacks(27), 27);

    // println!("\nBishop Debug");
    // debug_bitboard(movegen::bishop_attacks(27, 0), 27);

    // println!("\nRook Debug");
    // debug_bitboard(movegen::rook_attacks(26, 0), 26);

    // println!("\nQueen Debug");
    // debug_bitboard(movegen::queen_attacks(27, 0), 27);

    // println!("\nKing Debug");
    // debug_bitboard(movegen::king_attacks(27), 27);

    // println!("\nKing Debug");
    // debug_bitboard(movegen::king_attacks(7), 7);
}
