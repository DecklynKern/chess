mod game;
mod player;
mod hash;

use std::io::stdin;
use std::time;

fn main() {

    let line = get_line();
    let mut split = line.trim().split(" ");

    match split.next().unwrap() {
        "uci" => uci(),
        "perft" => perft(String::from("r3k2r/p1ppqpb1/bn2pnp1/3PN3/1p2P3/2N2Q2/PPPBBPpP/R3K2R b kq - 1 0"), split.next().unwrap().parse::<usize>().unwrap() as u64),
        _ => internal_sim()
    }

}
/*
use std::fs::OpenOptions;
use std::io::prelude::*;

fn log(string: &String) {
    
    let mut file = OpenOptions::new()
        .write(true)
        .append(true)
        .open("C:\\Users\\deckl\\OneDrive\\Desktop\\rust\\chess\\log.txt")
        .unwrap();

    if let Err(e) = write!(file, "{}", string) {
        eprintln!("Couldn't write to file: {}", e);
    }
}*/

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
    player = Box::new(player::AlphaBetaSearchPlayer::new(8));

    loop {

        let line = get_line();

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

                        let start_square = game::long_an_to_index(String::from(trimmed));
                        let end_square = game::long_an_to_index(trimmed.to_string()[2..4].to_string());
                        let diff = start_square.max(end_square) - start_square.min(end_square);
                        let piece = board.get_piece_abs(start_square);

                        board.make_move(&if piece.is_pawn() && diff == 24 {
                            if diff == 24 {
                                game::Move::new_pawn_double(&board, start_square, end_square)
                            } else if diff != 12 {
                                game::Move::new_en_passant(&board, start_square, end_square)
                            } else if end_square < 36 || end_square > 108 {
                                game::Move::new_promotion(&board, start_square, end_square, game::Piece::from_char(trimmed.to_string().chars().collect::<Vec<char>>()[5]))
                            } else {
                                game::Move::new(&board, start_square, end_square) // necessary duplicate to cover all cases
                            }
                        } else if piece.is_king() && diff == 2 {
                            game::Move::new_castle(&board, start_square, end_square)
                        } else {
                            game::Move::new(&board, start_square, end_square)
                        });

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
                    },
                    None => {}
                }
                
            },
            "stop" => {}, // cope?
            "ponderhit" => {},
            "quit" => break,
            _ => {}
        }
    }
}

fn perft(fen: String, depth: u64) {
 
    let start_time = time::Instant::now();

    let mut board = game::Board::from_fen(fen);

    let mut total_moves = 0;

    for mv in game::get_possible_moves(&board) {
        board.make_move(&mv);
        let next_moves = game::get_num_moves(&mut board, depth - 1);
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
        #[cfg(feature = "random")]
        "r" => {println!("random player");Box::new(player::RandomPlayer{})},
        "b" => Box::new(player::BasicSearchPlayer::new(4)),
        _ => Box::new(player::AlphaBetaSearchPlayer::new(4))
    };

    println!("enter p2 ('h' -> human, 'r' -> random, 'b' -> basicsearch, otherwise alphabeta): ");

    line = get_line();

    p2 = match line.trim() {
        "h" => Box::new(player::HumanPlayer{}),
        #[cfg(feature = "random")]
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