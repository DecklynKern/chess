use super::*;
use crate::player::Player;

#[test]
fn shannon_number_1ply() {
    assert_eq!(get_num_moves(&mut game::Board::from_fen(String::from("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1")), 2), 400);
}

#[test]
fn shannon_number_2ply() {
    assert_eq!(get_num_moves(&mut game::Board::from_fen(String::from("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1")), 4), 197281);
}

#[test]
fn shannon_number_3ply() {
    assert_eq!(get_num_moves(&mut game::Board::from_fen(String::from("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1")), 6), 119060324);
}


#[test]
fn position_2_2ply() {
    assert_eq!(get_num_moves(&mut game::Board::from_fen(String::from("r3k2r/p1ppqpb1/bn2pnp1/3PN3/1p2P3/2N2Q1p/PPPBBPPP/R3K2R w KQkq - ")), 4), 4085603);
}

#[test]
fn position_3_2ply() {
    assert_eq!(get_num_moves(&mut game::Board::from_fen(String::from("8/2p5/3p4/KP5r/1R3p1k/8/4P1P1/8 w - - ")), 4), 4085603);
}

#[test]
fn position_6_2ply() {
    assert_eq!(get_num_moves(&mut game::Board::from_fen(String::from("r4rk1/1pp1qppp/p1np1n2/2b1p1B1/2B1P1b1/P1NP1N2/1PP1QPPP/R4RK1 w - - 0 10 ")), 4), 3894594);
}

#[test]
fn capture_queen() {
    let mut board = game::Board::from_fen(String::from("rnb1kbnr/pppppppp/8/8/3Qq3/8/PPPPPPPP/RNB1KBNR w KQkq - 0 1"));
    let mut alphabeta = player::AlphaBetaSearchPlayer::new(6);
    let possible_moves = game::get_possible_moves(&board);
    assert_eq!(alphabeta.get_move(&mut board, &possible_moves).unwrap().to_long_an(), "d4e4");
}

#[test]
fn hashing() {

    let zobrist = hash::Zobrist::new();
    let mut table = hash::HashTable::new();
    let mut board = game::Board::default();

    let hash = zobrist.get_board_hash(&board);
    table.set(hash, 1000);
    assert_eq!(table.get(hash), Some(1000));

    let test_move = game::Move::new(&board, 101, 89);
    board.make_move(&test_move);
    board.undo_move();
    assert_eq!(table.get(zobrist.get_board_hash(&board)), Some(1000));
}