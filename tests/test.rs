extern crate chess;

#[test]
fn shannon_number_1ply() {
    assert_eq!(chess::game::get_num_moves(&mut chess::game::Board::from_fen(String::from("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1")), 2), 400);
}

#[test]
fn shannon_number_2ply() {
    assert_eq!(chess::game::get_num_moves(&mut chess::game::Board::from_fen(String::from("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1")), 4), 197281);
}

#[test]
fn shannon_number_3ply() {
    assert_eq!(chess::game::get_num_moves(&mut chess::game::Board::from_fen(String::from("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1")), 6), 119060324);
}

#[test]
fn position_2_2ply() {
    assert_eq!(chess::game::get_num_moves(&mut chess::game::Board::from_fen(String::from("r3k2r/p1ppqpb1/bn2pnp1/3PN3/1p2P3/2N2Q1p/PPPBBPPP/R3K2R w KQkq - ")), 4), 4085603);
}

// disabled until i fix en-passant
#[test]
#[ignore = "have not implemented en-passant pins"]
fn position_3_2ply() {
    assert_eq!(chess::game::get_num_moves(&mut chess::game::Board::from_fen(String::from("8/2p5/3p4/KP5r/1R3p1k/8/4P1P1/8 w - - ")), 4), 4085603);
}

#[test]
fn position_6_2ply() {
    assert_eq!(chess::game::get_num_moves(&mut chess::game::Board::from_fen(String::from("r4rk1/1pp1qppp/p1np1n2/2b1p1B1/2B1P1b1/P1NP1N2/1PP1QPPP/R4RK1 w - - 0 10 ")), 4), 3894594);
}

#[test]
fn capture_queen() {
    let mut board = chess::game::Board::from_fen(String::from("rnb1kbnr/pppppppp/8/8/3Qq3/8/PPPPPPPP/RNB1KBNR w KQkq - 0 1"));
    let mut alphabeta: Box<dyn chess::player::Player> = Box::new(chess::player::AlphaBetaPlayer::new(6, &chess::player::basic_eval));
    let possible_moves = chess::game::get_possible_moves(&board);
    assert_eq!(alphabeta.get_move(&mut board, &possible_moves).unwrap().to_long_an(), "d4e4");
}

#[test]
fn hashing() {

    let zobrist = chess::hash::Zobrist::new();
    let mut table = chess::hash::HashTable::new();
    let mut board = chess::game::Board::default();

    let hash = zobrist.get_board_hash(&board);
    table.set(hash, 1000);
    assert_eq!(table.get(hash), Some(&1000));

    let test_move = chess::game::Move::new(&board, chess::game::D2, chess::game::D3);
    board.make_move(&test_move);
    board.undo_move();
    assert_eq!(table.get(zobrist.get_board_hash(&board)), Some(&1000));
}

#[test]
fn hashing_update() {

    let zobrist = chess::hash::Zobrist::new();
    let mut board = chess::game::Board::from_fen(String::from("r3k2r/p1ppqpb1/bn2pnp1/3PN3/1p2P3/2N2Q1p/PPPBBPPP/R3K2R w KQkq - "));

    let mut hash = zobrist.get_board_hash(&board);
    let original_hash = hash;

    let old_en_passant_chance1 = board.en_passant_chance;
    let mut old_castling_rights1 = board.castling_rights;
    
    let move1 = chess::game::Move::new_castle(&board, chess::game::E1, chess::game::G1);
    board.make_move(&move1);

    hash = zobrist.update_hash(hash, &move1, old_en_passant_chance1, old_castling_rights1, board.castling_rights);
    old_castling_rights1 = board.castling_rights;

    let old_en_passant_chance2 = board.en_passant_chance;
    let mut old_castling_rights2 = board.castling_rights;

    let move2 = chess::game::Move::new_pawn_double(&board, chess::game::C7, chess::game::C5);
    board.make_move(&move2);

    hash = zobrist.update_hash(hash, &move2, old_en_passant_chance2, old_castling_rights2, board.castling_rights);
    old_castling_rights2 = board.castling_rights;

    board.undo_move();
    hash = zobrist.update_hash(hash, &move2, old_en_passant_chance2, old_castling_rights2, board.castling_rights);

    board.undo_move();
    hash = zobrist.update_hash(hash, &move1, old_en_passant_chance1, old_castling_rights1, board.castling_rights);
    assert_eq!(hash, original_hash);

}