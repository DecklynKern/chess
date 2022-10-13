use crate::player::player;
use crate::simulator::board;
use crate::simulator::piece;
use crate::simulator::eval;
use crate::hash;

pub struct BasicSearchPlayer {
    depth: usize,
    zobrist_hasher: hash::zobrist::Zobrist,
    transposition_table: hash::hashtable::HashTable<isize>,
    nodes_searched: usize
}

impl BasicSearchPlayer {

    pub fn new(depth: usize) -> BasicSearchPlayer {
        BasicSearchPlayer{
            depth: depth,
            zobrist_hasher: hash::zobrist::Zobrist::new(),
            transposition_table: hash::hashtable::HashTable::new(),
            nodes_searched: 0
        }
    }

    pub fn score_board(board: &board::Board) -> isize {
        let mut sum = 0;
        for square in board::VALID_SQUARES {
            let piece = board.get_piece_abs(square);
            sum += match piece {
                piece::WhitePawn => 100,
                piece::BlackPawn => -100,
                piece::WhiteKnight => 300,
                piece::BlackKnight => -300,
                piece::WhiteBishop => 300,
                piece::BlackBishop => -300,
                piece::WhiteRook => 500,
                piece::BlackRook => -500,
                piece::WhiteQueen => 900,
                piece::BlackQueen => -900,
                piece::WhiteKing => 0,
                piece::BlackKing => 0,
                piece::Empty => 0,
                piece::Border => 0
            };
        }
        return sum  * (if board.side_to_move == piece::Colour::White {1} else {-1});
    }

    fn find_move_score(&mut self, move_to_check: &eval::Move, board: &mut board::Board, depth: usize) -> isize {

        board.make_move(&move_to_check);
        self.nodes_searched += 1;

        let hash = self.zobrist_hasher.get_board_hash(board);

        return match self.transposition_table.get(hash) {
            Some(score) => {
                board.undo_move();
                score
            },
            None => {
                let score: isize;

                if depth == 0 {
                    score = -BasicSearchPlayer::score_board(&board);
                
                } else {
                    score = match eval::get_possible_moves(board).iter().map(|mv|
                        -self.find_move_score(mv, board, depth - 1)
                    ).min() {
                        Some(val) => val,
                        None => player::MAX_SCORE
                    };
                }
        
                board.undo_move();

                self.transposition_table.set(hash, score);
                score
            }
        };
    }
}

impl player::Player for BasicSearchPlayer {
    fn get_move<'a>(&mut self, board: &mut board::Board, possible_moves: &'a Vec<eval::Move>) -> Option<&'a eval::Move> {
        
        let mut best_move = None;
        let mut best_score = player::MIN_SCORE;
        let mut score: isize;

        self.transposition_table.clear();
        
        for possible_move in possible_moves {

            score = self.find_move_score(&possible_move, board, self.depth - 1);

            if score > best_score {
                best_score = score;
                best_move = Some(possible_move);
            }
        }

        println!("nodes searched: {}", self.nodes_searched);

        return best_move;

    }
}