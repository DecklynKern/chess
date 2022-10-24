use crate::player::*;
use crate::game;
use crate::hash;

const PAWN_VALUE: i64 = 100;
const KNIGHT_VALUE: i64 = 320;
const BISHOP_VALUE: i64 = 330;
const ROOK_VALUE: i64 = 530;
const QUEEN_VALUE: i64 = 960;

const KING_NEAR_EDGE_VALUE: i64 = 75;
const KING_NEAR_EDGE_FALLOFF: f64 = 0.6;

fn basic_move_ordering(moves: Vec<game::Move>) -> Vec<game::Move> {

    let mut sorted_moves = Vec::new();

    for possible_move in &moves {
        if possible_move.replaced_piece != game::Empty {
            sorted_moves.push(*possible_move);
        }
    }

    for possible_move in &moves {
        if possible_move.replaced_piece == game::Empty {
            sorted_moves.push(*possible_move);
        }
    }

    return sorted_moves;

}

pub struct AlphaBetaSearchPlayer {
    depth: u64,
    zobrist_hasher: hash::Zobrist,
    transposition_table: hash::HashTable<i64>,
    nodes_searched: u64
}

impl AlphaBetaSearchPlayer {

    pub fn new(depth: u64) -> AlphaBetaSearchPlayer {
        AlphaBetaSearchPlayer{
            depth: depth,
            zobrist_hasher: hash::Zobrist::new(),
            transposition_table: hash::HashTable::new(),
            nodes_searched: 0
        }
    }

    pub fn score_board(board: &game::Board) -> i64 {

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

    fn find_board_score(&mut self, board: &mut game::Board, depth: u64, mut alpha: i64, beta: i64, board_hash: u64) -> (i64, Option<game::Move>) {

        self.nodes_searched += 1;

        let mut score: i64;

        if depth == 0 {
            score = AlphaBetaSearchPlayer::score_board(&board);
            self.transposition_table.set(board_hash, score);
            return (score, None);
        }

        score = MIN_SCORE;

        let possible_moves = game::get_possible_moves(board);

        if possible_moves.is_empty() { // could also hash these i guess
            
            if game::get_king_attackers(board, board.side_to_move).is_empty() {
                return (0, None);
            }
            
            return (score, None);

        }

        let mut best_move = None;

        for possible_move in basic_move_ordering(possible_moves) {

            let old_en_passant_chance = board.en_passant_chance;
            let old_castling_rights = board.castling_rights;

            board.make_move(&possible_move);

            let board_hash = self.zobrist_hasher.update_hash(
                board_hash,
                &possible_move,
                old_en_passant_chance,
                old_castling_rights,
                board.castling_rights
            );

            let move_score = match self.transposition_table.get(board_hash) {
                Some(cached_score) => cached_score,
                _ => {
                    let move_score = -self.find_board_score(board, depth - 1, -beta, -alpha, board_hash).0;
                    self.transposition_table.set(board_hash, move_score);
                    move_score
                }
            };

            board.undo_move();

            if move_score > score {
                best_move = Some(possible_move);
                score = move_score;
            }

            alpha = alpha.max(score);

            if score >= beta {
                break;
            }
        }

        return (score, best_move);

    }
}

impl Player for AlphaBetaSearchPlayer {
    fn get_move<'a>(&mut self, board: &mut game::Board, possible_moves: &'a Vec<game::Move>) -> Option<&'a game::Move> {

        self.transposition_table.clear();

        let board_hash = self.zobrist_hasher.get_board_hash(board);

        let (_, best_move) = self.find_board_score(board, self.depth, MIN_SCORE, MAX_SCORE, board_hash);

        // println!("nodes searched: {}", self.nodes_searched);

        match best_move {
            Some(valid_move) => {
                for possible_move in possible_moves {
                    if possible_move.start_square == valid_move.start_square && possible_move.end_square == valid_move.end_square {
                        return Some(possible_move)
                    }
                }
                return None
            },
            None => return None
        }
    }
}