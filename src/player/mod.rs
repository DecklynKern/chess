use std::io::stdin;
use rand::seq::SliceRandom;
use rand::thread_rng;
use crate::simulator::board;
use crate::simulator::piece;
use crate::simulator::eval;
use crate::hash;

pub trait Player {
    fn get_move<'a>(&mut self, board: &mut board::Board, possible_moves: &'a Vec<eval::Move>) -> &'a eval::Move;
}

pub struct HumanPlayer {
}

impl Player for HumanPlayer {
    fn get_move<'a>(&mut self, _board: &mut board::Board, possible_moves: &'a Vec<eval::Move>) -> &'a eval::Move {
        
        let stdin = stdin();
        let mut line;

        println!("Possible moves: ");

        for possible_move in possible_moves.clone() {
            print!("{}, ", possible_move.to_an(&possible_moves));
        }
        println!();
        
        loop {

            println!("Select a move: ");
    
            line = String::new();
            stdin.read_line(&mut line).unwrap();   
            
            line = line.trim().to_string();
            
            for possible_move in possible_moves {
                if line == possible_move.to_an(&possible_moves) {
                    return &possible_move;
                }
            }

            println!("Invalid move.")

        }
    }
}

pub struct RandomPlayer {
}

impl Player for RandomPlayer {
    fn get_move<'a>(&mut self, _board: &mut board::Board, possible_moves: &'a Vec<eval::Move>) -> &'a eval::Move {
        let mut rng = thread_rng();
        return possible_moves.choose(&mut rng).unwrap();
    }
}

pub struct BasicSearchPlayer {
    depth: usize,
    zobrist_hasher: hash::zobrist::Zobrist,
    transposition_table: hash::hashtable::HashTable<isize>
}

impl BasicSearchPlayer {

    pub fn new(depth: usize) -> BasicSearchPlayer {
        BasicSearchPlayer{
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

    fn find_move_score(&mut self, move_to_check: &eval::Move, board: &mut board::Board, depth: usize) -> isize {

        board.make_move(&move_to_check);

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
                        None => 99999
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
    fn get_move<'a>(&mut self, board: &mut board::Board, possible_moves: &'a Vec<eval::Move>) -> &'a eval::Move {
        
        let mut best_move = &possible_moves[0];
        let mut best_score = -999999999isize;
        let mut score: isize;

        self.transposition_table.clear();
        
        for possible_move in possible_moves {

            score = self.find_move_score(&possible_move, board, self.depth - 1);

            if score > best_score {
                best_score = score;
                best_move = possible_move;
            }
        }

        return best_move;

    }
}