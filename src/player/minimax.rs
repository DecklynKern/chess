use crate::player::*;
use crate::game;
use crate::hash;

pub struct MiniMaxPlayer {
    depth: usize,
    score_board: BoardScore,
    zobrist_hasher: hash::Zobrist,
    transposition_table: hash::HashTable<i32, 20, 4>,
    nodes_searched: usize
}

impl MiniMaxPlayer {

    pub fn new(depth: usize, score_board: BoardScore) -> Self {
        Self{
            depth,
            score_board,
            zobrist_hasher: hash::Zobrist::new(),
            transposition_table: hash::HashTable::new(),
            nodes_searched: 0
        }
    }

    fn find_move_score(&mut self, move_to_check: &game::Move, board: &mut game::Board, depth: usize) -> i32 {

        board.make_move(move_to_check);
        self.nodes_searched += 1;

        let hash = self.zobrist_hasher.get_board_hash(board);

        return match self.transposition_table.get(hash) {
            Some(&mut score) => {
                board.undo_move();
                score
            },
            None => {

                let score = if depth == 0 {
                    -(self.score_board)(board)

                } else {
                    match game::get_possible_moves(board).iter().map(|mv|
                        -self.find_move_score(mv, board, depth - 1)
                    )
                    .min() {
                        Some(val) => val,
                        None => MAX_SCORE
                    }
                };
        
                board.undo_move();

                self.transposition_table.set(hash, score);
                score
            }
        };
    }
}

impl Player for MiniMaxPlayer {
    fn get_move<'a>(&mut self, board: &mut game::Board, possible_moves: &'a [game::Move]) -> Option<&'a game::Move> {
        
        let mut best_move = None;
        let mut best_score = MIN_SCORE;
        let mut score: i32;

        self.transposition_table.clear();
        
        for possible_move in possible_moves {

            score = self.find_move_score(possible_move, board, self.depth - 1);

            if score > best_score {
                best_score = score;
                best_move = Some(possible_move);
            }
        }

        // println!("nodes searched: {}", self.nodes_searched);

        return best_move;

    }
}