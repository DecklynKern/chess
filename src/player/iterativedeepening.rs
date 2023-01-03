use crate::player::*;
use crate::game;
use crate::hash;

use std::time;

pub struct IterativeDeepening {
    max_depth: u64,
    score_board: BoardScore,
    zobrist_hasher: hash::Zobrist,
    transposition_table: hash::HashTable<i64>,
    pv_table: hash::HashTable<game::Move>,
    nodes_searched: u64
}

impl IterativeDeepening {

    pub fn new(max_depth: u64, score_board: BoardScore) -> Self {
        Self{
            max_depth,
            score_board,
            zobrist_hasher: hash::Zobrist::new(),
            transposition_table: hash::HashTable::new(),
            pv_table: hash::HashTable::new(),
            nodes_searched: 0
        }
    }

    fn find_board_score(&mut self, board: &mut game::Board, depth: u64, mut alpha: i64, beta: i64, board_hash: u64) -> (i64, Option<game::Move>) {

        self.nodes_searched += 1;

        let mut score: i64;

        if depth == 0 {
            score = (self.score_board)(board);
            self.transposition_table.set(board_hash, score);
            return (score, None);
        }

        score = MIN_SCORE;

        let mut possible_moves = game::get_possible_moves(board);

        if possible_moves.is_empty() { // could also hash these i guess
            
            if game::get_king_attackers(board, board.side_to_move).is_empty() {
                self.transposition_table.set(board_hash, 0);
                return (0, None);
            }
            
            self.transposition_table.set(board_hash, LOSE_SCORE);
            return (LOSE_SCORE, None);

        }

        let mut best_move = None;

        //let mut pv_id = 0;

        // if let Some(pv_move) = self.pv_table.get(board_hash) {
        //     possible_moves.insert(0, *pv_move);
        // }

        for possible_move in possible_moves {

            let old_en_passant_chance = board.en_passant_chance;
            let old_castling_rights = board.castling_rights;

            board.make_move(&possible_move);

            let new_hash = self.zobrist_hasher.update_hash(
                board_hash,
                &possible_move,
                old_en_passant_chance,
                old_castling_rights,
                board.castling_rights
            );

            let move_score = match self.transposition_table.get(new_hash) {
                Some(&cached_score) => cached_score,
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

        if let Some(pv_move) = best_move {
            self.pv_table.set(board_hash, pv_move);
        }

        return (score - 1 * score.signum(), best_move);

    }
}

impl Player for IterativeDeepening {
    fn get_move<'a>(&mut self, board: &mut game::Board, possible_moves: &'a [game::Move]) -> Option<&'a game::Move> {
        
        self.nodes_searched = 0;
        self.pv_table.clear();
        
        let board_hash = self.zobrist_hasher.get_board_hash(board);

        let mut eval;
        let mut best_move = None;

        for search_depth in 1..=self.max_depth {

            self.transposition_table.clear();
            let start_time = time::Instant::now();

            (eval, best_move) = self.find_board_score(board, search_depth, MIN_SCORE, MAX_SCORE, board_hash);

            println!("depth {}: {}ms", search_depth, (time::Instant::now() - start_time).as_millis());
            println!("best move: {}, eval: {}", best_move.unwrap().to_long_an(), eval);

        }

        // println!("nodes searched: {}", self.nodes_searched);
        // println!("eval: {}", eval as f64 / 100.0);

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