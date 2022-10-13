mod simulator;
mod player;
mod hash;
use core::num;
use std::env;

use std::io::stdin;
use std::fs::OpenOptions;
use std::io::prelude::*;

fn main() {

    let line = get_line();
    let mut split = line.trim().split(" ");

    match split.next().unwrap() {
        "uci" => uci(),
        "perft" => perft(split.next().unwrap().parse::<usize>().unwrap()),
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

    let mut board = simulator::board::Board::default();

    let mut player: Box<dyn player::player::Player>;
    player = Box::new(player::alphabetasearchplayer::AlphaBetaSearchPlayer::new(6));

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
                    board = simulator::board::Board::default();                   
                    match split.next() {
                        Some(arg) => assert_eq!(arg.trim(), "moves"),
                        None => continue
                    }
                    for move_to_play in split {
                        let trimmed = move_to_play.trim();
                        board.make_move(&simulator::eval::Move::new(
                            &board,
                            simulator::chess_util::long_an_to_index(String::from(trimmed)),
                            simulator::chess_util::long_an_to_index(trimmed.to_string()[2..4].to_string())
                        ));
                    }
                } else {
                    board = simulator::board::Board::from_fen(split.collect::<Vec<&str>>().join(" "));
                }
            },
            "go" => {
                let possible_moves = simulator::eval::get_possible_moves(&mut board);
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

fn perft(depth: usize) {
    
    let mut board = simulator::board::Board::default();
    //let mut board = simulator::board::Board::from_fen(String::from("rnbqkbnr/1ppp1ppp/p4P2/4p3/8/8/PPPPP1PP/RNBQKBNR b KQkq - 0 1"));

    let mut total_moves: usize = 0;
    
    for mv in simulator::eval::get_possible_moves(&board) {
        board.make_move(&mv);
        let num_moves = simulator::eval::get_num_moves(&mut board, depth - 1);
        total_moves += num_moves;
        println!("{} {}", mv.to_long_an(), num_moves);
        board.undo_move();
    }

    println!("\nTotal moves: {}", total_moves);

}

fn internal_sim() {
    
    let mut board = simulator::board::Board::default();

    let mut p1: Box<dyn player::player::Player>;
    let mut p2: Box<dyn player::player::Player>;

    println!("enter p1 ('h' -> human, 'r' -> random, 'b' -> basicsearch, otherwise alphabeta): ");

    let mut line = get_line();

    p1 = match line.trim() {
        "h" => Box::new(player::humanplayer::HumanPlayer{}),
        "r" => Box::new(player::randomplayer::RandomPlayer{}),
        "b" => Box::new(player::basicsearchplayer::BasicSearchPlayer::new(4)),
        _ => Box::new(player::alphabetasearchplayer::AlphaBetaSearchPlayer::new(4))
    };

    println!("enter p2 ('h' -> human, 'r' -> random, 'b' -> basicsearch, otherwise alphabeta): ");

    line = get_line();

    p2 = match line.trim() {
        "h" => Box::new(player::humanplayer::HumanPlayer{}),
        "r" => Box::new(player::randomplayer::RandomPlayer{}),
        "b" => Box::new(player::basicsearchplayer::BasicSearchPlayer::new(4)),
        _ => Box::new(player::alphabetasearchplayer::AlphaBetaSearchPlayer::new(4))
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

        let possible_moves = simulator::eval::get_possible_moves(&mut board);

        if possible_moves.is_empty() {
            break;
        }

        let move_to_make = if board.side_to_move == simulator::piece::Colour::White {
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