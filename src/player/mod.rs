use std::io::stdin;
use rand::seq::SliceRandom;
use rand::thread_rng;
use crate::simulator::board;
use crate::simulator::eval;

pub trait Player {
    fn get_move<'a>(&self, board: &board::Board, possible_moves: &'a Vec<eval::Move>) -> &'a eval::Move;
}

pub struct HumanPlayer {
}

impl Player for HumanPlayer {
    fn get_move<'a>(&self, _board: &board::Board, possible_moves: &'a Vec<eval::Move>) -> &'a eval::Move {
        
        let stdin = stdin();
        let mut line;

        for possible_move in possible_moves.clone() {
            print!("{}, ", possible_move.to_an(&possible_moves));
        }
        println!();
        
        loop {
    
            line = String::new();
            stdin.read_line(&mut line).unwrap();   
            
            line = line.trim().to_string();
            
            for possible_move in possible_moves {
                if line == possible_move.to_an(&possible_moves) {
                    return &possible_move;
                }
            }
        }
    }
}

pub struct RandomPlayer {
}

impl Player for RandomPlayer {
    fn get_move<'a>(&self, _board: &board::Board, possible_moves: &'a Vec<eval::Move>) -> &'a eval::Move {
        let mut rng = thread_rng();
        return possible_moves.choose(&mut rng).unwrap();
    }
}