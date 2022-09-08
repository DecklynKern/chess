mod board;
mod eval;
use std::io::{stdin};
use rand::seq::SliceRandom;
use rand::thread_rng;

fn main() {

    let fen = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1";

    let mut board = board::Board::from_fen(String::from(fen));

    let mut stdin = stdin();
    let mut line;

    let mut rng = thread_rng();

    loop {

        print_nice_board(&board);

        let possible_moves = eval::get_possible_moves(&mut board, false);

        match board.side_to_move {
            board::Colour::Black => {
                board.make_move(&possible_moves.choose(&mut rng).unwrap());
            }
            board::Colour::White => {

                for possible_move in possible_moves.clone() {
                    print!("{}, ", possible_move.to_an(&possible_moves));
                }
                println!();
        
                line = String::new();
                stdin.read_line(&mut line).unwrap();   
                
                line = line.trim().to_string();
                
                for possible_move in possible_moves.clone() {
                    if line == possible_move.to_an(&possible_moves) {
                        board.make_move(&possible_move);
                        break;
                    }
                }
            }
        }

        

    }

}

fn get_num_moves(board: &mut board::Board, depth: usize) -> usize {

    let possible_moves = eval::get_possible_moves(board, false);
    
    if depth == 1 {
        return possible_moves.len();
    }

    let mut num_moves = 0;

    for possible_move in &possible_moves {

        board.make_move(&possible_move);

        num_moves += get_num_moves(board, depth - 1);

        board.undo_move();

    }

    return num_moves;

}

fn print_nice_board(board: &board::Board) {
    for row in 0..8 {
        print!("{}  ", 8 - row);
        for col in 0..8 {
            print!("{} ", board.board[col + row * 8].to_char());
        }
        println!();
    }
    println!("   a b c d e f g h");
}