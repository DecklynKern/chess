mod simulator;
mod player;

use std::io::stdin;
use std::fs::OpenOptions;
use std::io::prelude::*;

fn main() {

    let stdin = stdin();
    let mut line = String::new();
    let mut split;

    stdin.read_line(&mut line);

    assert_eq!(line.trim(), "uci");

    println!("id name DeckChess");
    println!("id author Deck");

    println!("uciok");

    let mut debug = false;
    let mut board = simulator::board::Board::default();

    let player: Box<dyn player::Player>;
    player = Box::new(player::RandomPlayer{});

    loop {

        line = String::new();
        stdin.read_line(&mut line);
        
        let mut file = OpenOptions::new()
        .write(true)
        .append(true)
        .open("C:\\Users\\deckl\\OneDrive\\Desktop\\rust\\chess\\output.txt")
        .unwrap();

        if let Err(e) = write!(file, "{}", line) {
            eprintln!("Couldn't write to file: {}", e);
        }

        split = line.trim().split(" ");

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
                } else {
                    board = simulator::board::Board::from_fen(String::from(arg2));
                }
                match split.next() {
                    Some(arg) => assert_eq!(arg.trim(), "moves"),
                    None => continue
                }
                
                for move_to_play in split {
                    let trimmed = move_to_play.trim();
                    board.make_move(&simulator::eval::Move::new(
                        &board,
                        simulator::board::Board::long_an_to_index(String::from(trimmed)),
                        simulator::board::Board::long_an_to_index(trimmed.to_string()[2..4].to_string())
                    ));
                }
            },
            "go" => {
                let possible_moves = simulator::eval::get_possible_moves(&mut board, false);
                println!("bestmove {}", player.get_move(&board, &possible_moves).to_an(&possible_moves));
            },
            "stop" => {}, // cope?
            "ponderhit" => {},
            "quit" => break,
            _ => {}
        }

    }
}

/*

fn main() {

    let white_player: Box<dyn player::Player>;
    let black_player: Box<dyn player::Player>;

    let mut board = simulator::board::Board::default();

    white_player = Box::new(player::HumanPlayer{});
    black_player = Box::new(player::RandomPlayer{});

    loop {

        print_nice_board(&board);
        println!();

        let possible_moves = simulator::eval::get_possible_moves(&mut board, false);

        if possible_moves.is_empty() {
            break;
        }

        let move_to_play = match board.side_to_move {
            simulator::piece::Colour::Black => {
                black_player.get_move(&board, &possible_moves)
            }
            simulator::piece::Colour::White => {
                white_player.get_move(&board, &possible_moves)
            }
        };

        board.make_move(move_to_play);

        if move_to_play.replaced_piece != simulator::piece::Piece::Empty || move_to_play.special_move_type == simulator::eval::SpecialMoveType::EnPassant {

            let counts = board.get_piece_counts();

            match counts {
                (0, 0, 0, 0, 0, 2) | (0, 1, 0, 0, 0, 2) | (0, 0, 1, 0, 0, 2) => {
                    print_nice_board(&board);
                    break;
                },
                _ => {}
            }
        }
    }
}

fn print_nice_board(board: &simulator::board::Board) {
    for row in 0..8 {
        print!("{} ", 8 - row);
        for col in 0..8 {
            print!("|{}", board.board[col + row * 8].to_char());
        }
        println!("|");
    }
    println!("  ----------------");
    println!("   a b c d e f g h");
}*/