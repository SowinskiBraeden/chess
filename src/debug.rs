// ref: https://chessprogramming.org/Bitboards

use crate::chess::{Board, Color, Game, Piece, debug_bitboard};

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
    board.set_bitboard(Color::Black, Piece::Rook, 0);
    board.set_bitboard(Color::Black, Piece::Queen, 0);

    board.set_bitboard(Color::White, Piece::Pawn, 1u64 << 55); // one away from promotion
    board.set_bitboard(Color::White, Piece::Knight, 0);
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

fn test_checkmate_white(game: &mut Game) {
    println!("\nDebug checkmate white");

    let mut board = game.board;

    board.set_bitboard(Color::Black, Piece::Pawn, 0);
    board.set_bitboard(Color::Black, Piece::Knight, 0);
    board.set_bitboard(Color::Black, Piece::Bishop, 1u64 << 33);
    board.set_bitboard(Color::Black, Piece::Rook, 0);
    board.set_bitboard(Color::Black, Piece::Queen, 1u64 << 12);

    board.set_bitboard(Color::White, Piece::Pawn, 0);
    board.set_bitboard(Color::White, Piece::Knight, 0);
    board.set_bitboard(Color::White, Piece::Bishop, 0);
    board.set_bitboard(Color::White, Piece::Rook, 0);
    board.set_bitboard(Color::White, Piece::Queen, 0);

    game.board = board;
    let checkmate = game.checkmate();
    println!("{board}");
    debug_bitboard(board.moves(4), 4);
    println!("Checkmate: {checkmate:?}");
    println!("------------------");
}

fn test_checkmate_black(game: &mut Game) {
    println!("\nDebug checkmate black");

    let mut board = game.board;

    board.set_bitboard(Color::White, Piece::Pawn, 0);
    board.set_bitboard(Color::White, Piece::Knight, 0);
    board.set_bitboard(Color::White, Piece::Bishop, 1u64 << 33);
    board.set_bitboard(Color::White, Piece::Rook, 0);
    board.set_bitboard(Color::White, Piece::Queen, 1u64 << 12);

    board.set_bitboard(Color::Black, Piece::Pawn, 0);
    board.set_bitboard(Color::Black, Piece::Knight, 0);
    board.set_bitboard(Color::Black, Piece::Bishop, 0);
    board.set_bitboard(Color::Black, Piece::Rook, 0);
    board.set_bitboard(Color::Black, Piece::Queen, 0);
    board.set_bitboard(Color::Black, Piece::King, 1u64 << 4);

    game.board = board;
    game.board.turn = Color::Black; // checkmate looks at current turn
    let checkmate = game.checkmate();
    println!("{board}");
    debug_bitboard(board.moves(4), 4);
    println!("Checkmate: {checkmate:?}");
    println!("------------------");
}

fn test_checkmate_nope(game: &mut Game) {
    println!("\nDebug checkmate white nope can capture");

    let mut board = game.board;

    board.set_bitboard(Color::Black, Piece::Pawn, 0);
    board.set_bitboard(Color::Black, Piece::Knight, 0);
    board.set_bitboard(Color::Black, Piece::Bishop, 0);
    board.set_bitboard(Color::Black, Piece::Rook, 0);
    board.set_bitboard(Color::Black, Piece::Queen, 1u64 << 12);

    board.set_bitboard(Color::White, Piece::Pawn, 0);
    board.set_bitboard(Color::White, Piece::Knight, 0);
    board.set_bitboard(Color::White, Piece::Bishop, 0);
    board.set_bitboard(Color::White, Piece::Rook, 0);
    board.set_bitboard(Color::White, Piece::Queen, 0);

    game.board = board;
    let checkmate = game.checkmate();
    println!("{board}");
    debug_bitboard(board.moves(4), 4);
    println!("Checkmate: {checkmate:?}");
    println!("------------------");
}

fn test_checkmate_nope_2(game: &mut Game) {
    println!("\nDebug checkmate white nope can block");

    let mut board = game.board;

    board.set_bitboard(Color::Black, Piece::Pawn, 0);
    board.set_bitboard(Color::Black, Piece::Knight, 0);
    board.set_bitboard(Color::Black, Piece::Bishop, 0);
    board.set_bitboard(Color::Black, Piece::Rook, (1u64 << 43) | (1u64 << 45));
    board.set_bitboard(Color::Black, Piece::Queen, 1u64 << 44);

    board.set_bitboard(Color::White, Piece::Pawn, 0);
    board.set_bitboard(Color::White, Piece::Knight, 0);
    board.set_bitboard(Color::White, Piece::Bishop, 0);
    board.set_bitboard(Color::White, Piece::Rook, 1u64 << 16);
    board.set_bitboard(Color::White, Piece::Queen, 0);

    game.board = board;
    let checkmate = game.checkmate();
    println!("{board}");
    debug_bitboard(board.moves(4), 4);
    debug_bitboard(board.moves(16), 16);
    println!("Checkmate: {checkmate:?}");
    println!("------------------");
}

fn test_stalemate(game: &mut Game) {
    println!("\nDebug stalemate ");

    let mut board = game.board;

    board.set_bitboard(Color::Black, Piece::Pawn, 0);
    board.set_bitboard(Color::Black, Piece::Knight, 0);
    board.set_bitboard(Color::Black, Piece::Bishop, 0);
    board.set_bitboard(Color::Black, Piece::Rook, (1u64 << 43) | (1u64 << 45));
    board.set_bitboard(Color::Black, Piece::Queen, 1u64 << 8);

    board.set_bitboard(Color::White, Piece::Pawn, 0);
    board.set_bitboard(Color::White, Piece::Knight, 0);
    board.set_bitboard(Color::White, Piece::Bishop, 0);
    board.set_bitboard(Color::White, Piece::Rook, 0);
    board.set_bitboard(Color::White, Piece::Queen, 0);

    game.board = board;
    let stalemate = game.stalemate();
    println!("{board}");
    debug_bitboard(board.moves(4), 4);
    println!("Stalemate: {stalemate}");
    println!("------------------");
}

fn test_white_castle_king(board: &mut Board) {
    println!("\nDebug white caslte king side");

    board.set_bitboard(Color::White, Piece::Bishop, 0);
    board.set_bitboard(Color::White, Piece::Knight, 0);
    board.set_bitboard(Color::White, Piece::Queen, 0);
    println!("{board}");
    debug_bitboard(board.moves(4), 4);
    let _ = board.play_move(4, 6, None);
    println!("\n{board}");
    println!("------------------");
}

fn test_white_castle_queen(board: &mut Board) {
    println!("\nDebug white caslte queen side");

    board.set_bitboard(Color::White, Piece::Bishop, 0);
    board.set_bitboard(Color::White, Piece::Knight, 0);
    board.set_bitboard(Color::White, Piece::Queen, 0);
    println!("{board}");
    debug_bitboard(board.moves(4), 4);
    let _ = board.play_move(4, 2, None);
    println!("\n{board}");
    println!("------------------");
}

fn test_black_castle_king(board: &mut Board) {
    println!("\nDebug black caslte king side");

    board.set_bitboard(Color::Black, Piece::Bishop, 0);
    board.set_bitboard(Color::Black, Piece::Knight, 0);
    board.set_bitboard(Color::Black, Piece::Queen, 0);
    println!("{board}");
    board.turn = Color::Black;
    debug_bitboard(board.moves(60), 60);
    let _ = board.play_move(60, 62, None);
    println!("\n{board}");
    println!("------------------");
}

fn test_black_castle_queen(board: &mut Board) {
    println!("\nDebug black caslte queen side");

    board.set_bitboard(Color::Black, Piece::Bishop, 0);
    board.set_bitboard(Color::Black, Piece::Knight, 0);
    board.set_bitboard(Color::Black, Piece::Queen, 0);
    println!("{board}");
    board.turn = Color::Black;
    debug_bitboard(board.moves(60), 60);
    let _ = board.play_move(60, 58, None);
    println!("\n{board}");
    println!("------------------");
}

pub fn run() {
    test_legal_move_filters(&mut Board::new());

    test_castle_1(&mut Board::new());
    test_castle_2(&mut Board::new());
    test_castle_3(&mut Board::new());
    test_castle_4(&mut Board::new());
    test_castle_5(&mut Board::new());
    test_promotion(&mut Board::new(), Piece::Queen);
    test_promotion(&mut Board::new(), Piece::Pawn);

    test_checkmate_white(&mut Game::new());
    test_checkmate_black(&mut Game::new());
    test_checkmate_nope(&mut Game::new());
    test_checkmate_nope_2(&mut Game::new());

    test_stalemate(&mut Game::new());

    test_white_castle_king(&mut Board::new());
    test_white_castle_queen(&mut Board::new());
    test_black_castle_king(&mut Board::new());
    test_black_castle_queen(&mut Board::new());
}
