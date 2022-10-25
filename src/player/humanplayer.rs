use crate::player;
use crate::game;

use std::io::stdin;

pub struct HumanPlayer {
}

impl player::Player for HumanPlayer {
    fn get_move<'a>(&mut self, _board: &mut game::Board, possible_moves: &'a [game::Move]) -> Option<&'a game::Move> {

        if possible_moves.is_empty() {
            return None;
        }
        
        let stdin = stdin();
        let mut line;

        println!("Possible moves: ");

        for possible_move in possible_moves {
            print!("{}, ", possible_move.to_an(possible_moves));
        }
        println!();
        
        loop {

            println!("Select a move: ");
    
            line = String::new();
            stdin.read_line(&mut line).unwrap();   
            
            line = line.trim().to_string();
            
            for possible_move in possible_moves {
                if line == possible_move.to_an(possible_moves) {
                    return Some(possible_move);
                }
            }

            println!("Invalid move.")

        }
    }
}