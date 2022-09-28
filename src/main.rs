mod simulator;
mod player;
mod hash;

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

    let mut debug = false;
    let mut board = simulator::board::Board::default();

    let mut player: Box<dyn player::player::Player>;
    player = Box::new(player::alphabetasearchplayer::AlphaBetaSearchPlayer::new(6));

    loop {

        let line = get_line();
        
        log(&line);

        let mut split = line.trim().split(" ");

        match split.next().unwrap().trim() {
            "debug" => {
                match split.next().unwrap().trim() {
                    "on" => debug = true,
                    "off" => debug = false,
                    _ => {}
                }
            },
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
                let mv = player.get_move(&mut board, &possible_moves).to_long_an();
                println!("bestmove {}", mv);
                log(&String::from(format!(">>> bestmove {}\n", mv)));
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

    for n in 1..=depth {
        println!("{}: {}", n, simulator::eval::get_num_moves(&mut board, n));
    }

}

fn internal_sim() {
    
    let mut board = simulator::board::Board::default();

    let mut p1: Box<dyn player::player::Player>;
    let mut p2: Box<dyn player::player::Player>;

    println!("enter p1 ('h' -> human, 'r' -> random): ");

    let mut line = get_line();

    p1 = match line.trim() {
        "h" => Box::new(player::humanplayer::HumanPlayer{}),
        "r" => Box::new(player::randomplayer::RandomPlayer{}),
        _ => Box::new(player::alphabetasearchplayer::AlphaBetaSearchPlayer::new(4))
    };

    println!("enter p2 ('h' -> human, 'r' -> random): ");

    line = get_line();

    p2 = match line.trim() {
        "h" => Box::new(player::humanplayer::HumanPlayer{}),
        "r" => Box::new(player::randomplayer::RandomPlayer{}),
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

        println!("{} is played.\n", move_to_make.to_an(&possible_moves));

        board.make_move(move_to_make);

    }
}