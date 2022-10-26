use crate::game;

pub type BoardScore = &'static(dyn Fn(&game::Board) -> i64);

pub const MIN_SCORE: i64 = i64::MIN + 1;
pub const MAX_SCORE: i64 = i64::MAX;

const PAWN_VALUE: i64 = 100;
const KNIGHT_VALUE: i64 = 320;
const BISHOP_VALUE: i64 = 330;
const ROOK_VALUE: i64 = 530;
const QUEEN_VALUE: i64 = 960;

pub fn basic_eval(board: &game::Board) -> i64 {

    let mut score = 0;

    let white_pieces = board.get_piece_counts(game::White);
    let black_pieces = board.get_piece_counts(game::Black);

    score += PAWN_VALUE * (white_pieces.0 as i64 - black_pieces.0 as i64);
    score += KNIGHT_VALUE * (white_pieces.1 as i64 - black_pieces.1 as i64);
    score += BISHOP_VALUE * (white_pieces.2 as i64 - black_pieces.2 as i64);
    score += ROOK_VALUE * (white_pieces.3 as i64 - black_pieces.3 as i64);
    score += QUEEN_VALUE * (white_pieces.4 as i64 - black_pieces.4 as i64);

    score *= -board.side_to_move.to_dir() as i64;

    return score;

}

// thanks https://www.chessprogramming.org/Simplified_Evaluation_Function
const PAWN_SQUARE_VALUES: [i64; 144] = [
    0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,
    0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,
    0,  0,100,100,100,100,100,100,100,100,  0,  0,
    0,  0,150,150,150,150,150,150,150,150,  0,  0,
    0,  0,110,110,120,130,130,120,110,110,  0,  0,
    0,  0,105,105,110,125,125,110,105,105,  0,  0,
    0,  0,100,100,100,120,120,100,100,100,  0,  0,
    0,  0,105, 95, 90,100,100, 90, 95,105,  0,  0,
    0,  0,105,110,110, 80, 80,110,110,105,  0,  0,
    0,  0,100,100,100,100,100,100,100,100,  0,  0,
    0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,
    0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0
];

const KNIGHT_SQUARE_VALUES: [i64; 144] = [
    0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,
    0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,
    0,  0,270,280,290,290,290,290,280,270,  0,  0,
    0,  0,280,300,320,320,320,320,300,280,  0,  0,
    0,  0,290,320,330,335,335,330,320,290,  0,  0,
    0,  0,290,325,335,340,340,335,325,290,  0,  0,
    0,  0,290,320,335,340,340,335,320,290,  0,  0,
    0,  0,290,325,330,335,335,330,325,290,  0,  0,
    0,  0,280,300,320,325,325,320,300,280,  0,  0,
    0,  0,270,280,290,290,290,290,280,270,  0,  0,
    0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,
    0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0
];

const BISHOP_SQUARE_VALUES: [i64; 144] = [
    0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,
    0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,
    0,  0,310,320,320,320,320,320,320,310,  0,  0,
    0,  0,320,330,330,330,330,330,330,320,  0,  0,
    0,  0,320,330,335,340,340,335,330,320,  0,  0,
    0,  0,320,335,335,340,340,335,335,320,  0,  0,
    0,  0,320,330,340,340,340,340,330,320,  0,  0,
    0,  0,320,340,340,340,340,340,340,320,  0,  0,
    0,  0,320,335,330,330,330,330,335,320,  0,  0,
    0,  0,310,320,320,320,320,320,320,310,  0,  0,
    0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,
    0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0
];

const ROOK_SQUARE_VALUES: [i64; 144] = [
    0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,
    0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,
    0,  0,530,530,530,530,530,530,530,530,  0,  0,
    0,  0,535,540,540,540,540,540,540,535,  0,  0,
    0,  0,525,530,530,530,530,530,530,525,  0,  0,
    0,  0,525,530,530,530,530,530,530,525,  0,  0,
    0,  0,525,530,530,530,530,530,530,525,  0,  0,
    0,  0,525,530,530,530,530,530,530,525,  0,  0,
    0,  0,525,530,530,530,530,530,530,525,  0,  0,
    0,  0,530,530,530,535,535,530,530,530,  0,  0,
    0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,
    0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0
];

const QUEEN_SQUARE_VALUES: [i64; 144] = [
    0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,
    0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,
    0,  0,940,950,950,955,955,950,950,940,  0,  0,
    0,  0,950,960,960,960,960,960,960,950,  0,  0,
    0,  0,950,960,965,965,965,965,960,950,  0,  0,
    0,  0,955,960,965,965,965,965,960,955,  0,  0,
    0,  0,960,960,965,965,965,965,960,955,  0,  0,
    0,  0,950,965,965,965,965,965,960,950,  0,  0,
    0,  0,950,960,965,960,960,960,960,950,  0,  0,
    0,  0,940,950,950,955,955,950,950,940,  0,  0,
    0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,
    0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0
];

const KING_EARLY_SQUARE_VALUES: [i64; 144] = [
    0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,
    0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,
    0,  0,-30,-40,-40,-50,-50,-40,-40,-30,  0,  0,
    0,  0,-30,-40,-40,-50,-50,-40,-40,-30,  0,  0,
    0,  0,-30,-40,-40,-50,-50,-40,-40,-30,  0,  0,
    0,  0,-30,-40,-40,-50,-50,-40,-40,-30,  0,  0,
    0,  0,-20,-30,-30,-40,-40,-30,-30,-20,  0,  0,
    0,  0,-10,-20,-20,-20,-20,-20,-20,-10,  0,  0,
    0,  0, 20, 20,  0,  0,  0,  0, 20, 20,  0,  0,
    0,  0, 20, 30, 10,  0,  0, 10, 30, 20,  0,  0,
    0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,
    0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0
];

const KING_LATE_SQUARE_VALUES: [i64; 144] = [
    0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,
    0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,
    0,  0,-30,-40,-40,-50,-50,-40,-40,-30,  0,  0,
    0,  0,-30,-40,-40,-50,-50,-40,-40,-30,  0,  0,
    0,  0,-30,-40,-40,-50,-50,-40,-40,-30,  0,  0,
    0,  0,-30,-40,-40,-50,-50,-40,-40,-30,  0,  0,
    0,  0,-20,-30,-30,-40,-40,-30,-30,-20,  0,  0,
    0,  0,-10,-20,-20,-20,-20,-20,-20,-10,  0,  0,
    0,  0, 20, 20,  0,  0,  0,  0, 20, 20,  0,  0,
    0,  0, 20, 30, 10,  0,  0, 10, 30, 20,  0,  0,
    0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,
    0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0
];

fn flip(piece: usize) -> usize {
    143 - piece
}

pub fn advanced_eval(board: &game::Board) -> i64 {

    let mut score = 0;

    for &piece in &board.piece_positions[(game::WHITE | game::PAWN) as usize] {
        score += PAWN_SQUARE_VALUES[piece];
    }

    for &piece in &board.piece_positions[(game::BLACK | game::PAWN) as usize] {
        score -= PAWN_SQUARE_VALUES[flip(piece)];
    }

    for &piece in &board.piece_positions[(game::WHITE | game::KNIGHT) as usize] {
        score += KNIGHT_SQUARE_VALUES[piece];
    }

    for &piece in &board.piece_positions[(game::BLACK | game::KNIGHT) as usize] {
        score -= KNIGHT_SQUARE_VALUES[flip(piece)];
    }

    for &piece in &board.piece_positions[(game::WHITE | game::BISHOP) as usize] {
        score += BISHOP_SQUARE_VALUES[piece];
    }

    for &piece in &board.piece_positions[(game::BLACK | game::BISHOP) as usize] {
        score -= BISHOP_SQUARE_VALUES[flip(piece)];
    }

    for &piece in &board.piece_positions[(game::WHITE | game::ROOK) as usize] {
        score += ROOK_SQUARE_VALUES[piece];
    }

    for &piece in &board.piece_positions[(game::BLACK | game::ROOK) as usize] {
        score -= ROOK_SQUARE_VALUES[flip(piece)];
    }

    for &piece in &board.piece_positions[(game::WHITE | game::QUEEN) as usize] {
        score += QUEEN_SQUARE_VALUES[piece];
    }

    for &piece in &board.piece_positions[(game::BLACK | game::QUEEN) as usize] {
        score -= QUEEN_SQUARE_VALUES[flip(piece)];
    }

    // add late game stuff later
    score += KING_EARLY_SQUARE_VALUES[board.white_king];
    score -= KING_EARLY_SQUARE_VALUES[flip(board.black_king)];

    score *= -board.side_to_move.to_dir() as i64;

    return score;

}