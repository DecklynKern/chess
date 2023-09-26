use std::io::{BufRead, BufReader, BufWriter, Write};
use std::process::{Command, Stdio};

fn main() {

    println!("{}", (std::env::current_dir().unwrap()).display());

    let mut player1 = Command::new("./src/bin/versions/chess-current")
        .stdin(Stdio::piped())
        .spawn()
        .unwrap();

    let mut player2 = Command::new("./src/bin/versions/chess-current")
        .stdin(Stdio::piped())
        .spawn()
        .unwrap();

    let mut player1_stdin = player1.stdin.unwrap();
    let mut player1_writer = BufWriter::new(&mut player1_stdin);

    let mut player2_stdin = player2.stdin.unwrap();
    let mut player2_writer = BufWriter::new(&mut player2_stdin);

}