mod simulator;
mod player;

fn main() {

    let white_player: Box<dyn player::Player>;
    let black_player: Box<dyn player::Player>;

    let mut board = simulator::board::Board::default();

    white_player = Box::new(player::RandomPlayer{});
    black_player = Box::new(player::HumanPlayer{});

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

            println!("{} {} {} {} {} {}", counts.0, counts.1, counts.2, counts.3, counts.4, counts.5);

            match counts {
                (0, 0, 0, 0, 0, 2) | (0, 1, 0, 0, 0, 2) | (0, 0, 1, 0, 0, 2) => {
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
}