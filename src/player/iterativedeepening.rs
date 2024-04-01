use crate::player::*;
use crate::game;
use crate::hash;

use std::time;

pub struct IterativeDeepening {
    max_time_millis: u128,
    score_board: BoardScore,
    zobrist_hasher: hash::Zobrist,
    transposition_table: hash::HashTable<i32, 20, 4>,
    pv_table: hash::HashTable<game::Move, 20, 4>,
    nodes_searched: u32
}

impl IterativeDeepening {

    pub fn new(approx_time_millis: u128, score_board: BoardScore) -> Self {
        Self{
            max_time_millis: approx_time_millis,
            score_board,
            zobrist_hasher: hash::Zobrist::new(),
            transposition_table: hash::HashTable::new(),
            pv_table: hash::HashTable::new(),
            nodes_searched: 0
        }
    }

    fn find_board_score(&mut self, board: &mut game::Board, depth: u32, mut alpha: i32, beta: i32, board_hash: u64) -> (i32, Option<game::Move>) {

        self.nodes_searched += 1;

        let mut score: i32;

        if depth == 0 {
            score = (self.score_board)(board);
            self.transposition_table.set(board_hash, score);
            return (score, None);
        }

        score = MIN_SCORE;

        let mut possible_moves = game::get_possible_moves(board);

        if possible_moves.is_empty() { // could also hash these i guess
            
            if game::get_position_info(board, board.side_to_move).king_attacker_count == 0 {
                self.transposition_table.set(board_hash, 0);
                return (0, None);
            }
            
            self.transposition_table.set(board_hash, LOSE_SCORE);
            return (LOSE_SCORE, None);

        }

        if let Some(pv_move) = self.pv_table.get(board_hash) {

            for (idx, possible_move) in possible_moves.iter().enumerate() {

                if possible_move.start_square == pv_move.start_square && 
                possible_move.end_square == pv_move.end_square {
                    possible_moves.swap(0, idx);
                    break;
                }
            }
        }

        let mut best_move = None;

        for possible_move in possible_moves {

            let old_en_passant_chance = board.en_passant_chance;
            let old_castling_rights = board.castling_rights;

            let new_hash = self.zobrist_hasher.update_hash(
                board_hash,
                &possible_move,
                old_en_passant_chance,
                old_castling_rights,
                board.castling_rights
            );

            let move_score = if let Some(&mut cached_score) = self.transposition_table.get(new_hash) {
                cached_score
            }
            else {

                board.make_move(&possible_move);
                
                let move_score = -self.find_board_score(board, depth - 1, -beta, -alpha, board_hash).0;
                self.transposition_table.set(board_hash, move_score);
                
                board.undo_move();
                move_score
                
            };

            if move_score > score {
                best_move = Some(possible_move);
                score = move_score;
            }

            alpha = alpha.max(score);

            if score >= beta {
                break;
            }
        }

        if let Some(pv_move) = &best_move {
            self.pv_table.set(board_hash, pv_move.clone());
        }

        (score - score.signum(), best_move)

    }
}

impl Player for IterativeDeepening {
    fn get_move<'a>(&mut self, board: &mut game::Board, possible_moves: &'a [game::Move]) -> Option<&'a game::Move> {
        
        self.nodes_searched = 0;
        self.pv_table.clear();
        
        let board_hash = self.zobrist_hasher.get_board_hash(board);

        let mut eval = 0;
        let mut best_move = None;

        let mut time_taken = 0;

        let mut search_depth = 1;
        let start_time = time::Instant::now();

        while time_taken < self.max_time_millis {

            self.transposition_table.clear();

            (eval, best_move) = self.find_board_score(board, search_depth, MIN_SCORE, MAX_SCORE, board_hash);

            // println!("depth {}: {}ms", search_depth, (time::Instant::now() - start_time).as_millis());
            // println!("best move: {}, eval: {}", best_move.unwrap().to_long_an(), eval);

            search_depth += 1;
            time_taken = (time::Instant::now() - start_time).as_millis();

        }

        println!("nodes searched: {}", self.nodes_searched);
        println!("total depth: {}", search_depth);
        // println!("eval: {}", eval as f64 / 100.0);

        best_move.and_then(|valid_move| {
            possible_moves.iter().find(|possible_move| possible_move.start_square == valid_move.start_square && possible_move.end_square == valid_move.end_square)
        })
    }
}
