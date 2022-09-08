mod board;
mod eval;

fn main() {

    let fen = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1";

    let mut board = board::Board::from_fen(String::from(fen));

    get_num_moves(&mut board, 3);
    
    for n in 1..=10 {
        println!("{} {}", n, get_num_moves(&mut board, n));
    }

}

fn get_num_moves(board: &mut board::Board, depth: usize) -> usize {

    let possible_moves = eval::get_possible_moves(&board);
    
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