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

const KING_NEAR_EDGE_VALUE: i64 = 75;
const KING_NEAR_EDGE_FALLOFF: f64 = 0.6;

pub fn advanced_eval(board: &game::Board) -> i64 {

    let mut score = 0;

    let white_pieces = board.get_piece_counts(game::White);
    let black_pieces = board.get_piece_counts(game::Black);

    score += PAWN_VALUE * (white_pieces.0 as i64 - black_pieces.0 as i64);
    score += KNIGHT_VALUE * (white_pieces.1 as i64 - black_pieces.1 as i64);
    score += BISHOP_VALUE * (white_pieces.2 as i64 - black_pieces.2 as i64);
    score += ROOK_VALUE * (white_pieces.3 as i64 - black_pieces.3 as i64);
    score += QUEEN_VALUE * (white_pieces.4 as i64 - black_pieces.4 as i64);

    score *= -board.side_to_move.to_dir() as i64;

    /*
    let (opp_king_rank, opp_king_file) = game::Board::pos_to_row_col(match board.side_to_move {
        game::Colour::White => board.black_king,
        game::Colour::Black => board.white_king
    });

    let king_dist_to_corner = (4 - opp_king_rank as isize) + (4 - opp_king_file as isize);*/

    return score;

}