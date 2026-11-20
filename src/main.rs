// ref: https://chessprogramming.org/Bitboards

mod chess;
use chess::{Board, Color, Piece, debug_bitboard};

fn main() {
    let mut game = Board::new();

    game.set_bitboard(Color::Black, Piece::Bishop, 1u64 << 25); // puts enemy bishop in sight of king on queen pawn move
    game.set_bitboard(
        Color::White,
        Piece::Pawn,
        game.pieces[Color::White as usize][Piece::Pawn as usize] & !(1u64 << 11),
    );
    game.set_bitboard(
        Color::White,
        Piece::Knight,
        game.pieces[Color::White as usize][Piece::Knight as usize] | (1u64 << 19),
    );

    println!("{game}");

    println!("\nDebug Black Attacks");
    debug_bitboard(game.all_attacks(Color::Black), 25);

    println!("\nDebug White King in check");
    debug_bitboard(game.moves(4), 4);

    println!("\nDebug White knight can save");
    debug_bitboard(game.moves(19), 19);

    println!("\nDebug White pawn can save");
    debug_bitboard(game.moves(10), 10);

    println!("\nDebug white pawn cannot save");
    debug_bitboard(game.moves(9), 9);

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
