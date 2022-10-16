mod game;
mod player;
mod hash;

#[cfg(test)]
mod test;

use std::io::stdin;
use std::fs::OpenOptions;
use std::io::prelude::*;
use std::time;

fn main() {

    let line = get_line();
    let mut split = line.trim().split(" ");

    match split.next().unwrap() {
        "uci" => uci(),
        "perft" => perft(String::from("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1"), split.next().unwrap().parse::<usize>().unwrap() as u64),
        _ => internal_sim()
    }

}

fn log(string: &String) {
    
    let mut file = OpenOptions::new()
        .write(true)
        .append(true)
        .open("C:\\Users\\deckl\\OneDrive\\Desktop\\rust\\chess\\log.txt")
        .unwrap();

    if let Err(e) = write!(file, "{}", string) {
        eprintln!("Couldn't write to file: {}", e);
    }
}

fn get_line() -> String {
    let stdin = stdin();
    let mut line = String::new();
    stdin.read_line(&mut line).expect("line reading failed");
    return line;
}

fn uci() {

    println!("id name DeckChess");
    println!("id author Deck");

    let mut board = game::Board::default();

    let mut player: Box<dyn player::Player>;
    player = Box::new(player::AlphaBetaSearchPlayer::new(1));

    loop {

        let line = get_line();
        
        log(&line);

        let mut split = line.trim().split(" ");

        match split.next().unwrap().trim() {
            "isready" => println!("readyok"),
            "setoption" => {}, // can change
            "register" => {}, // ?
            "ucinewgame" => {}, //shrug
            "position" => {
                let arg2 = split.next().unwrap().trim();
                if arg2 == "startpos" {
                    board = game::Board::default();                   
                    match split.next() {
                        Some(arg) => assert_eq!(arg.trim(), "moves"),
                        None => continue
                    }
                    for move_to_play in split {
                        let trimmed = move_to_play.trim();
                        board.make_move(&game::Move::new(
                            &board,
                            game::long_an_to_index(String::from(trimmed)),
                            game::long_an_to_index(trimmed.to_string()[2..4].to_string())
                        ));
                    }
                } else {
                    board = game::Board::from_fen(split.collect::<Vec<&str>>().join(" "));
                }
            },
            "go" => {
                let possible_moves = game::get_possible_moves(&mut board);
                let mv = player.get_move(&mut board, &possible_moves);
                match mv {
                    Some(valid_move) => {
                        let move_text = valid_move.to_long_an();
                        println!("bestmove {}", move_text);
                        log(&String::from(format!(">>> bestmove {}\n", move_text)));
                    },
                    None => {log(&String::from("### game over"))}
                }
                
            },
            "stop" => {}, // cope?
            "ponderhit" => {},
            "quit" => break,
            _ => {}
        }

    }
}

pub fn get_num_moves(board: &mut game::Board, depth: u64) -> u64 {

    if depth == 0 {
        return 1;
    }

    let possible_moves = game::get_possible_moves(board);
    
    if depth == 1 {
        return possible_moves.len() as u64;
    }

    let mut moves = 0;

    for possible_move in &possible_moves {
        board.make_move(&possible_move);
        moves += get_num_moves(board, depth - 1);
        board.undo_move();
    }

    return moves;

}

fn perft(fen: String, depth: u64) {
 
    let start_time = time::Instant::now();

    let mut board = game::Board::from_fen(fen);

    let mut total_moves = 0;

    for mv in game::get_possible_moves(&board) {
        board.make_move(&mv);
        let next_moves = get_num_moves(&mut board, depth - 1);
        total_moves += next_moves;
        println!("{}: {}", mv.to_long_an(), next_moves);
        board.undo_move();
    }

    let end_time = time::Instant::now();

    let diff = (end_time - start_time).as_millis() as u64;

    println!();
    println!("Total time : {}ms", diff);
    println!("Total moves: {}", total_moves);

}

fn internal_sim() {
    
    let mut board = game::Board::default();

    let mut p1: Box<dyn player::Player>;
    let mut p2: Box<dyn player::Player>;

    println!("enter p1 ('h' -> human, 'r' -> random, 'b' -> basicsearch, otherwise alphabeta): ");

    let mut line = get_line();

    p1 = match line.trim() {
        "h" => Box::new(player::HumanPlayer{}),
        "r" => Box::new(player::RandomPlayer{}),
        "b" => Box::new(player::BasicSearchPlayer::new(4)),
        _ => Box::new(player::AlphaBetaSearchPlayer::new(4))
    };

    println!("enter p2 ('h' -> human, 'r' -> random, 'b' -> basicsearch, otherwise alphabeta): ");

    line = get_line();

    p2 = match line.trim() {
        "h" => Box::new(player::HumanPlayer{}),
        "r" => Box::new(player::RandomPlayer{}),
        "b" => Box::new(player::BasicSearchPlayer::new(4)),
        _ => Box::new(player::AlphaBetaSearchPlayer::new(4))
    };

    loop {

        for row in 0..8 {
            for col in 0..8 {
                print!("{} ", board.get_piece(row, col).to_char());
            }
            println!("")
        }
        
        line = get_line();     

        if line.trim() == "undo" {
            board.undo_move();
            continue;
        }

        let possible_moves = game::get_possible_moves(&mut board);

        if possible_moves.is_empty() {
            break;
        }

        let move_to_make = if board.side_to_move == game::Colour::White {
            p1.get_move(&mut board, &possible_moves)

        } else {
            p2.get_move(&mut board, &possible_moves)
        };

        match move_to_make {
            Some(valid_move) => {
                println!("{} is played.\n", valid_move.to_an(&possible_moves));
                board.make_move(valid_move);
            },
            None => {println!("game over")}
        }
    }
}