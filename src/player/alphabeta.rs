use crate::player::*;
use crate::game;
use crate::hash;

fn basic_move_ordering(moves: Vec<game::Move>) -> Vec<game::Move> {

    let mut sorted_moves = Vec::new();

    for possible_move in &moves {
        if possible_move.replaced_piece != game::Empty {
            sorted_moves.push(possible_move.clone());
        }
    }

    for possible_move in &moves {
        if possible_move.replaced_piece == game::Empty {
            sorted_moves.push(possible_move.clone());
        }
    }

    return sorted_moves;

}

pub struct AlphaBetaPlayer {
    depth: u32,
    score_board: BoardScore,
    zobrist_hasher: hash::Zobrist,
    transposition_table: hash::HashTable<i32>,
    nodes_searched: u32
}

impl AlphaBetaPlayer {

    pub fn new(depth: u32, score_board: BoardScore) -> Self {
        Self{
            depth,
            score_board,
            zobrist_hasher: hash::Zobrist::new(),
            transposition_table: hash::HashTable::new(),
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

        let possible_moves = game::get_possible_moves(board);

        if possible_moves.is_empty() {
            
            if game::get_king_attackers(board, board.side_to_move).is_empty() {
                return (0, None);
            }
            
            return (LOSE_SCORE, None);

        }

        let mut best_move = None;

        for possible_move in basic_move_ordering(possible_moves) {

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

        return (score - score.signum(), best_move);

    }
}

impl Player for AlphaBetaPlayer {
    fn get_move<'a>(&mut self, board: &mut game::Board, possible_moves: &'a [game::Move]) -> Option<&'a game::Move> {

        self.transposition_table.clear();
        self.nodes_searched = 0;

        let board_hash = self.zobrist_hasher.get_board_hash(board);

        let (eval, best_move) = self.find_board_score(board, self.depth, MIN_SCORE, MAX_SCORE, board_hash);

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