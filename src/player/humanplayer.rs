use crate::player::player;
use std::io::stdin;
use crate::simulator::board;
use crate::simulator::eval;

pub struct HumanPlayer {
}

impl player::Player for HumanPlayer {
    fn get_move<'a>(&mut self, _board: &mut board::Board, possible_moves: &'a Vec<eval::Move>) -> Option<&'a eval::Move> {

        if possible_moves.len() == 0 {
            return None;
        }
        
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
                    return Some(&possible_move);
                }
            }

            println!("Invalid move.")

        }
    }
}