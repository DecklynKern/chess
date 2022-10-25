use crate::player::*;
use crate::game;
use crate::hash;

pub struct BasicSearchPlayer {
    depth: usize,
    zobrist_hasher: hash::Zobrist,
    transposition_table: hash::HashTable<i64>,
    nodes_searched: usize
}

impl BasicSearchPlayer {

    pub fn new(depth: usize) -> BasicSearchPlayer {
        BasicSearchPlayer{
            depth,
            zobrist_hasher: hash::Zobrist::new(),
            transposition_table: hash::HashTable::new(),
            nodes_searched: 0
        }
    }

    pub fn score_board(board: &game::Board) -> i64 {
        let mut sum = 0;
        for square in game::VALID_SQUARES {
            let piece = board.get_piece_abs(square);
            sum += match piece {
                game::WhitePawn => 100,
                game::BlackPawn => -100,
                game::WhiteKnight => 300,
                game::BlackKnight => -300,
                game::WhiteBishop => 300,
                game::BlackBishop => -300,
                game::WhiteRook => 500,
                game::BlackRook => -500,
                game::WhiteQueen => 900,
                game::BlackQueen => -900,
                game::WhiteKing => 0,
                game::BlackKing => 0,
                game::Empty => 0,
                game::Border => 0
            };
        }
        return sum  * (if board.side_to_move == game::Colour::White {1} else {-1});
    }

    fn find_move_score(&mut self, move_to_check: &game::Move, board: &mut game::Board, depth: usize) -> i64 {

        board.make_move(&move_to_check);
        self.nodes_searched += 1;

        let hash = self.zobrist_hasher.get_board_hash(board);

        return match self.transposition_table.get(hash) {
            Some(score) => {
                board.undo_move();
                score
            },
            None => {
                let score: i64;

                if depth == 0 {
                    score = -BasicSearchPlayer::score_board(&board);
                
                } else {
                    score = match game::get_possible_moves(board).iter().map(|mv|
                        -self.find_move_score(mv, board, depth - 1)
                    ).min() {
                        Some(val) => val,
                        None => MAX_SCORE
                    };
                }
        
                board.undo_move();

                self.transposition_table.set(hash, score);
                score
            }
        };
    }
}

impl Player for BasicSearchPlayer {
    fn get_move<'a>(&mut self, board: &mut game::Board, possible_moves: &'a Vec<game::Move>) -> Option<&'a game::Move> {
        
        let mut best_move = None;
        let mut best_score = MIN_SCORE;
        let mut score: i64;

        self.transposition_table.clear();
        
        for possible_move in possible_moves {

            score = self.find_move_score(&possible_move, board, self.depth - 1);

            if score > best_score {
                best_score = score;
                best_move = Some(possible_move);
            }
        }

        // println!("nodes searched: {}", self.nodes_searched);

        return best_move;

    }
}