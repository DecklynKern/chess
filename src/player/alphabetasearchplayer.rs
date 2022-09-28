use crate::player::player;
use crate::simulator::board;
use crate::simulator::piece;
use crate::simulator::eval;
use crate::hash;

pub struct AlphaBetaSearchPlayer {
    depth: usize,
    zobrist_hasher: hash::zobrist::Zobrist,
    transposition_table: hash::hashtable::HashTable<isize>
}

impl AlphaBetaSearchPlayer {

    pub fn new(depth: usize) -> AlphaBetaSearchPlayer {
        AlphaBetaSearchPlayer{
            depth: depth,
            zobrist_hasher: hash::zobrist::Zobrist::new(),
            transposition_table: hash::hashtable::HashTable::new()
        }
    }

    pub fn score_board(board: &board::Board) -> isize {
        let mut sum = 0;
        for square in board::VALID_SQUARES {
            let piece = board.get_piece_abs(square);
            sum += match piece {
                piece::Piece::Pawn{colour: _} => 100,
                piece::Piece::Knight{colour: _} => 300,
                piece::Piece::Bishop{colour: _} => 300,
                piece::Piece::Rook{colour: _} => 500,
                piece::Piece::Queen{colour: _} => 900,
                piece::Piece::King{colour: _} => 0,
                piece::Piece::Empty => 0,
                piece::Piece::Border => 0
            } * (if piece.get_colour() == board.side_to_move {1} else {-1});
        }
        return sum;
    }

    fn find_move_score(&mut self, move_to_check: &eval::Move, board: &mut board::Board, depth: usize, mut alpha: isize, mut beta: isize, is_minimizing: bool) -> isize {

        board.make_move(&move_to_check);

        let hash = self.zobrist_hasher.get_board_hash(board);

        let move_score = match self.transposition_table.get(hash) {
            Some(score) => score,
            None => {

                let mut score: isize;

                if depth == 0 {

                    score = if is_minimizing {
                        -AlphaBetaSearchPlayer::score_board(&board)
                    } else {
                        AlphaBetaSearchPlayer::score_board(&board)
                    };
                
                } else {

                    if is_minimizing {

                        score = player::MAX_SCORE;

                        let response_moves = eval::get_possible_moves(board);
    
                        if response_moves.is_empty() {
                            if !eval::is_in_check(board) {
                                score = 0;
                            }
    
                        } else {
    
                            for response_move in response_moves {
    
                                score = (self.find_move_score(&response_move, board, depth - 1, alpha, beta, false)).min(score);
                                beta = score.min(beta);
    
                                if score <= alpha {
                                    break;
                                }
                            }
                        }
                    
                    } else {

                        score = player::MIN_SCORE;

                        let response_moves = eval::get_possible_moves(board);
    
                        if response_moves.is_empty() {
                            if !eval::is_in_check(board) {
                                score = 0;
                            }
    
                        } else {
    
                            for response_move in response_moves {
    
                                score = (self.find_move_score(&response_move, board, depth - 1, alpha, beta, true)).max(score);
                                alpha = score.max(alpha);
    
                                if score >= beta {
                                    break;
                                }
                            }
                        }
                    }
                }

                self.transposition_table.set(hash, score);
                score
                
            }
        };

        board.undo_move();
        return move_score;

    }
}

impl player::Player for AlphaBetaSearchPlayer {
    fn get_move<'a>(&mut self, board: &mut board::Board, possible_moves: &'a Vec<eval::Move>) -> &'a eval::Move {
        
        let mut best_move = &possible_moves[0];
        let mut best_score = player::MIN_SCORE;
        let mut score: isize;

        self.transposition_table.clear();
        
        for possible_move in possible_moves {

            score = self.find_move_score(&possible_move, board, self.depth - 1, player::MIN_SCORE, player::MAX_SCORE, true);

            if score > best_score {
                best_score = score;
                best_move = possible_move;
            }
        }

        return best_move;

    }
}