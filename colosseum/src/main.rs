use std::io::*;
use std::process::*;
use std::fs::File;

use chess::game::{Board, Move, get_possible_moves};
use chrono::Local;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum GameResult {
    Win,
    Draw,
    Loss
}

impl GameResult {
    pub fn as_score(self) -> &'static str {
        match self {
            Self::Win => "1-0",
            Self::Draw => "1/2-1/2",
            Self::Loss => "0-1"
        }
    }
}

fn main() {

    println!("{}", (std::env::current_dir().unwrap()).display());
    
    let p1_name = "chess-current-5s";
    let p2_name = "chess-current";

    let p1 = Command::new(format!("./versions/{}", p1_name))
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()
        .unwrap();

    let p2 = Command::new(format!("./versions/{}", p2_name))
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()
        .unwrap();

    let p1_stdin = p1.stdin.unwrap();
    let p1_stdout = p1.stdout.unwrap();
    
    let p2_stdin = p2.stdin.unwrap();
    let p2_stdout = p2.stdout.unwrap();
    
    let mut moves = String::new();
    let result = run(p1_stdin, p1_stdout, p2_stdin, p2_stdout, &mut moves);
    
    let now = Local::now();
    let mut file = File::create(format!("games/{}.pgn", now.format("%Y%m%d-%H%M"))).unwrap();
    
    writeln!(file, "[Date \"{}\"]", now.format("%Y.%m.%d"));
    writeln!(file, "[White \"{}\"]", p1_name);
    writeln!(file, "[Black \"{}\"]", p2_name);
    writeln!(file, "[Result \"{}\"]", result.as_score());
    writeln!(file);
    
    writeln!(file, "{}{}", moves, result.as_score());
    
}

fn run(
    p1_stdin: ChildStdin, mut p1_stdout: ChildStdout,
    p2_stdin: ChildStdin, mut p2_stdout: ChildStdout,
    moves: &mut String
) -> GameResult {
    
    let mut player1_writer = BufWriter::new(&p1_stdin);
    let mut player1_reader = BufReader::new(&mut p1_stdout);
    let mut player2_writer = BufWriter::new(&p2_stdin);
    let mut player2_reader = BufReader::new(&mut p2_stdout);
    
    let mut buf = String::new();
    
    writeln!(player1_writer, "uci");
    player1_writer.flush();
    
    writeln!(player2_writer, "uci");
    player2_writer.flush();
    
    player1_reader.read_line(&mut buf);
    player1_reader.read_line(&mut buf);
    
    player2_reader.read_line(&mut buf);
    player2_reader.read_line(&mut buf);
    
    let mut p1_turn = true;
    
    let mut board = Board::default();
    
    loop {
        
        if board.turns_taken == 200 {
            return GameResult::Draw;
        }
        
        if p1_turn {
            moves.push_str(&(board.turns_taken / 2 + 1).to_string());
            moves.push_str(". ");
        }
        
        let (reader, writer) = if p1_turn {
            (&mut player1_reader, &mut player1_writer)
        }
        else {
            (&mut player2_reader, &mut player2_writer)
        };
        
        writeln!(writer, "position fen {}", board.get_as_fen());
        writeln!(writer, "go");
        writer.flush();
        
        let mut response = String::new();
        reader.read_line(&mut response);
        
        if response == "resign\n" {
            return if p1_turn {
                GameResult::Loss
            }
            else {
                GameResult::Win    
            }
        }
        else {
            
            let long_an = response.chars()
                .skip("bestmove ".len())
                .take("a1b1".len())
                .collect::<String>();
        
            let move_to_make = Move::from_long_an(&long_an, &board);
            let possible_moves = get_possible_moves(&board);
            
            board.make_move(&move_to_make);
            
            let move_an = move_to_make.to_an(&possible_moves);
            
            moves.push_str(&move_an);
            moves.push(' ');
            
            println!("move made: {}", move_an);
            
        }
        
        if board.is_draw_by_insufficient_material() {
            return GameResult::Draw;
        }
        
        p1_turn = !p1_turn;
        
    }
}