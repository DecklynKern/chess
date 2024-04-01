#![allow(clippy::needless_return)]
mod game;
mod player;
mod hash;

use std::io::stdin;
use std::time;

fn main() {

    game::load_move_boards();

    let line = get_line();
    let mut split = line.trim().split(' ');

    match split.next().unwrap() {
        "uci" => uci(),
        "perft" => perft(String::from("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1"), split.next().unwrap().parse::<usize>().unwrap() as u32),
        _ => internal_sim()
    }
}

fn get_line() -> String {
    let stdin = stdin();
    let mut line = String::new();
    stdin.read_line(&mut line).expect("line reading failed");
    return line;
}

fn uci() {

    println!("id name Chess");
    println!("id author DecklynKern");

    let mut board = game::Board::default();

    let mut player: Box<dyn player::Player>;
    // player = Box::new(player::AlphaBetaPlayer::new(8, &player::advanced_eval));
    player = Box::new(player::IterativeDeepening::new(500, &player::advanced_eval));

    loop {

        let line = get_line();

        let mut split = line.trim().split(' ');

        match split.next().unwrap().trim() {
            "d" => print_board(&board),
            "isready" => println!("readyok"),
            "setoption" => {}, // can change
            "register" => {}, // ?
            "ucinewgame" => {}, //?
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

                        // remove magic numbers potentially
                        board.make_move(&game::Move::from_long_an(trimmed, &board));
                        
                    }
                }
                else {
                    board = game::Board::from_fen(split.collect::<Vec<&str>>().join(" "));
                }
            },
            "go" => {
                let possible_moves = game::get_possible_moves(&board);
                if let Some(valid_move) = player.get_move(&mut board, &possible_moves) {
                    let move_text = valid_move.to_long_an();
                    println!("bestmove {}", move_text);
                }
                else {
                    println!("resign");   
                }
            }
            "fen" => {
                println!("{}", board.get_fen());
            }
            "stop" => {}, // ?
            "ponderhit" => {},
            "quit" => break,
            _ => {}
        }
    }
}

fn perft(fen: String, depth: u32) {
 
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

    let diff = (end_time - start_time).as_millis() as u32;

    println!();
    println!("Total time : {}ms", diff);
    println!("Total moves: {}", total_moves);

}

fn print_board(board: &game::Board) {
    for row in 0..8 {
        for col in 0..8 {
            print!("{} ", board.get_piece_rc(row, col).to_char());
        }
        println!()
    }
}

fn internal_sim() {
    
    let mut board = game::Board::default();

    let mut p1: Box<dyn player::Player>;
    let mut p2: Box<dyn player::Player>;

    println!("enter p1 ('h' -> human, 'r' -> random, 'b' -> basicsearch, 'a' -> alphabeta, otherwise iterdeep): ");

    let mut line = get_line();

    p1 = match line.trim() {
        "h" => Box::new(player::HumanPlayer{}),
        #[cfg(feature = "random")]
        "r" => {println!("random player");Box::new(player::RandomPlayer{})},
        "b" => Box::new(player::MiniMaxPlayer::new(4, &player::basic_eval)),
        "a" => Box::new(player::AlphaBetaPlayer::new(4, &player::advanced_eval)),
        _ => Box::new(player::IterativeDeepening::new(1500, &player::advanced_eval))
    };

    println!("enter p2 ('h' -> human, 'r' -> random, 'b' -> basicsearch, 'a' -> alphabeta, otherwise iterdeep): ");

    line = get_line();

    p2 = match line.trim() {
        "h" => Box::new(player::HumanPlayer{}),
        #[cfg(feature = "random")]
        "r" => Box::new(player::RandomPlayer{}),
        "b" => Box::new(player::MiniMaxPlayer::new(4, &player::basic_eval)),
        "a" => Box::new(player::AlphaBetaPlayer::new(4, &player::advanced_eval)),
        _ => Box::new(player::IterativeDeepening::new(5000, &player::advanced_eval))
    };

    loop {

        print_board(&board);
        
        line = get_line();     

        if line.trim() == "undo" {
            board.undo_move();
            continue;
        }

        let possible_moves = game::get_possible_moves(&board);

        if possible_moves.is_empty() {
            break;
        }

        let move_to_make = if board.side_to_move == game::Colour::White {
            p1.get_move(&mut board, &possible_moves)
        }
        else {
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