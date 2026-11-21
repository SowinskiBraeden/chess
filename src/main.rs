// ref: https://chessprogramming.org/Bitboards

mod chess;
use chess::{Board, Color, Piece, debug_bitboard};

fn test_legal_move_filters(board: &mut Board) {
    board.set_bitboard(Color::Black, Piece::Bishop, 1u64 << 25); // puts enemy bishop in sight of king on queen pawn move
    board.set_bitboard(
        Color::White,
        Piece::Pawn,
        board.pieces[Color::White as usize][Piece::Pawn as usize] & !(1u64 << 11),
    );
    board.set_bitboard(
        Color::White,
        Piece::Knight,
        board.pieces[Color::White as usize][Piece::Knight as usize] | (1u64 << 19),
    );

    println!("{board}");

    println!("\nDebug Black Attacks");
    debug_bitboard(board.all_attacks(Color::Black), 25);

    println!("\nDebug White King in check");
    debug_bitboard(board.moves(4), 4);

    println!("\nDebug White knight can save");
    debug_bitboard(board.moves(19), 19);

    println!("\nDebug White pawn can save");
    debug_bitboard(board.moves(10), 10);

    println!("\nDebug white pawn cannot save");
    debug_bitboard(board.moves(9), 9);
    println!("------------------");
}

fn test_castle_1(board: &mut Board) {
    println!("\nDebug castle both ways");
    board.set_bitboard(Color::White, Piece::Bishop, 0);
    board.set_bitboard(Color::White, Piece::Knight, 0);
    board.set_bitboard(Color::White, Piece::Queen, 0);
    println!("{board}");
    debug_bitboard(board.moves(4), 4);
    println!("------------------");
}

fn test_castle_2(board: &mut Board) {
    println!("\nDebug no castling (rights but blocked)");
    board.set_bitboard(Color::White, Piece::Bishop, 0);
    board.set_bitboard(Color::White, Piece::Queen, 0);
    println!("{board}");
    debug_bitboard(board.moves(4), 4);
    println!("------------------");
}

fn test_castle_3(board: &mut Board) {
    println!("\nDebug no castle kingside rights");
    board.set_bitboard(Color::White, Piece::Pawn, 0);
    board.set_bitboard(Color::White, Piece::Bishop, 0);
    board.set_bitboard(Color::White, Piece::Knight, 0);
    board.set_bitboard(Color::White, Piece::Queen, 0);
    let _ = board.play_move(7, 15, None); // move kingside rook up 1
    board.turn = Color::White; // force turn back to white for valid move gen
    println!("{board}");
    debug_bitboard(board.moves(4), 4);
    println!("------------------");
}

fn test_castle_4(board: &mut Board) {
    println!("\nDebug no castle queenside rights");
    board.set_bitboard(Color::White, Piece::Pawn, 0);
    board.set_bitboard(Color::White, Piece::Bishop, 0);
    board.set_bitboard(Color::White, Piece::Knight, 0);
    board.set_bitboard(Color::White, Piece::Queen, 0);
    let _ = board.play_move(0, 8, None); // move kingside rook
    board.turn = Color::White; // force turn back to white for valid move gen
    println!("{board}");
    debug_bitboard(board.moves(4), 4);
    println!("------------------");
}

fn test_castle_5(board: &mut Board) {
    println!("\nDebug no castling, no rights from king move");
    board.set_bitboard(Color::White, Piece::Pawn, 0);
    board.set_bitboard(Color::White, Piece::Bishop, 0);
    board.set_bitboard(Color::White, Piece::Knight, 0);
    board.set_bitboard(Color::White, Piece::Queen, 0);
    let _ = board.play_move(4, 3, None); // move king to left
    board.turn = Color::White; // force turn back to white for valid move gen
    println!("{board}");
    debug_bitboard(board.moves(3), 3);
    println!("------------------");
}

fn test_promotion(board: &mut Board, piece: Piece) {
    println!("\nDebug promotion");

    board.set_bitboard(Color::Black, Piece::Pawn, 0);
    board.set_bitboard(Color::Black, Piece::Knight, 0);
    board.set_bitboard(Color::Black, Piece::Bishop, 0);
    board.set_bitboard(Color::Black, Piece::Bishop, 0);
    board.set_bitboard(Color::Black, Piece::Rook, 0);
    board.set_bitboard(Color::Black, Piece::Queen, 0);

    board.set_bitboard(Color::White, Piece::Pawn, 1u64 << 55); // one away from promotion
    board.set_bitboard(Color::White, Piece::Knight, 0);
    board.set_bitboard(Color::White, Piece::Bishop, 0);
    board.set_bitboard(Color::White, Piece::Bishop, 0);
    board.set_bitboard(Color::White, Piece::Rook, 0);
    board.set_bitboard(Color::White, Piece::Queen, 0);

    println!("{board}");
    let r = board.play_move(55, 63, Some(piece));
    println!("{board}");
    if r.is_err() {
        println!("invalid promotion");
    };
    println!("------------------");
}

fn test_promotion_fail(board: &mut Board) {
    println!("\nDebug promotion fail");

    board.set_bitboard(Color::Black, Piece::Pawn, 0);
    board.set_bitboard(Color::Black, Piece::Knight, 0);
    board.set_bitboard(Color::Black, Piece::Bishop, 0);
    board.set_bitboard(Color::Black, Piece::Bishop, 0);
    board.set_bitboard(Color::Black, Piece::Queen, 0);

    board.set_bitboard(Color::White, Piece::Pawn, 1u64 << 55); // one away from promotion
    board.set_bitboard(Color::White, Piece::Knight, 0);
    board.set_bitboard(Color::White, Piece::Bishop, 0);
    board.set_bitboard(Color::White, Piece::Bishop, 0);
    board.set_bitboard(Color::White, Piece::Queen, 0);

    println!("{board}");
    let _ = board.play_move(55, 64, Some(Piece::Knight));
    println!("{board}");
    println!("------------------");
}

fn main() {
    let mut board = Board::new();
    test_legal_move_filters(&mut board);

    board = Board::new();
    test_castle_1(&mut board);
    board = Board::new();
    test_castle_2(&mut board);
    board = Board::new();
    test_castle_3(&mut board);
    board = Board::new();
    test_castle_4(&mut board);
    board = Board::new();
    test_castle_5(&mut board);

    board = Board::new();
    test_promotion(&mut board, Piece::Queen);

    board = Board::new();
    test_promotion(&mut board, Piece::Pawn);
}
