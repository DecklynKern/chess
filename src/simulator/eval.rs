use crate::simulator::piece::*;
use crate::simulator::board::*;

#[derive(Clone, Copy)]
pub struct Move {
    pub start_square: usize,
    pub end_square: usize,
    pub moved_piece: Piece,
    pub replaced_piece: Piece,
    pub old_en_passant_chance: Option<usize>,
    pub old_castling_rights : (bool, bool, bool, bool),
    pub is_en_passant: bool,
    pub is_castle: bool
}

impl Move {

    pub fn new(board: &Board, start_square: usize, end_square: usize) -> Move {

        let moved_piece = board.board[start_square];
        let replaced_piece = board.board[end_square];

        Move {
            start_square: start_square,
            end_square: end_square,
            moved_piece: moved_piece,
            replaced_piece: replaced_piece,
            old_en_passant_chance: board.en_passant_chance,
            old_castling_rights: board.castling_rights,
            is_en_passant: match moved_piece {
                Piece::Pawn{colour: _} => {((start_square as isize - end_square as isize).abs() - 8).abs() == 1 && replaced_piece == Piece::Empty},
                _ => false
            },
            is_castle: match moved_piece {
                Piece::King{colour: _} => {(start_square as isize - end_square as isize).abs() == 2},
                _ => false
            }
        }
    }

    pub fn to_an(&self, possible_moves: &Vec<Move>) -> String {

        let mut same_dest_moves: Vec<&Move> = Vec::new();

        for possible_move in possible_moves {
            if possible_move.moved_piece == self.moved_piece && possible_move.end_square == self.end_square && possible_move.start_square != self.start_square {
                same_dest_moves.push(possible_move);
            }

        }

        if self.is_castle {
            return String::from(match self.end_square {
                2 => "o-o-o",
                6 => "o-o",
                58 => "O-O-O",
                62 => "O-O",
                _ => "" // should not happen
            })
        }

        return format!("{}{}{}{}",
            match self.moved_piece {
                Piece::Pawn {colour: _} => String::from(""),
                piece => piece.to_char().to_string(),
            },
            if same_dest_moves.is_empty() {
                String::from("")
            } else {
                Board::index_to_an(self.start_square)
            },
            if self.replaced_piece != Piece::Empty || self.is_en_passant {"x"} else {""},
            Board::index_to_an(self.end_square)
        );
    }
}

#[derive(PartialEq)]
enum AddResult {
    Capture,
    Move,
    Fail
}

#[derive(PartialEq)]
enum MoveType{
    NonCapture,
    Capture,
    Move
}

fn invalid_square(start_square: usize, row_offset: isize, col_offset: isize) -> bool {

    let col = (start_square % 8) as isize + col_offset;
    let row = (start_square / 8) as isize + row_offset;

    return col < 0 || col > 7 || row < 0 || row > 7;

}

fn try_add_move(moves: &mut Vec<Move>, board: &Board, start_square: usize, row_offset: isize, col_offset: isize, move_type: MoveType) -> AddResult {

    if invalid_square(start_square, row_offset, col_offset) {
        return AddResult::Fail;
    }

    let end_square = (start_square as isize + row_offset * 8 + col_offset) as usize;
    let end_piece = board.board[end_square];
    let end_piece_empty = end_piece == Piece::Empty;

    if (!end_piece_empty && board.board[start_square].get_colour() == end_piece.get_colour()) ||
        (move_type == MoveType::Capture && end_piece_empty) ||
        (move_type == MoveType::NonCapture && !end_piece_empty) {
        return AddResult::Fail;
    }

    let new_move = Move::new(&board, start_square, end_square);
    let result = if new_move.replaced_piece != Piece::Empty {AddResult::Capture} else {AddResult::Move};

    moves.push(new_move);

    return result;

}

fn add_sliding_moves(moves: &mut Vec<Move>, board: &Board, start_square: usize, orthogonal: bool, diagonal: bool) {

    let mut move_dirs: Vec<(isize, isize)> = Vec::new();

    if orthogonal {
        move_dirs.push((0, -1));
        move_dirs.push((0, 1));
        move_dirs.push((-1, 0));
        move_dirs.push((1, 0));
    }

    if diagonal {
        move_dirs.push((-1, -1));
        move_dirs.push((-1, 1));
        move_dirs.push((1, -1));
        move_dirs.push((1, 1));
    }

    for (row_offset, col_offset) in move_dirs {

        let mut num_slides = 0;

        loop {

            num_slides += 1;

            if try_add_move(moves, board, start_square, row_offset * num_slides, col_offset * num_slides, MoveType::Move) != AddResult::Move {
                break;
            }

        }
    }
}

pub fn get_possible_moves(board: &mut Board, ignore_check: bool) -> Vec<Move> {

    let side_to_move = board.side_to_move;
    let mut moves: Vec<Move> = Vec::new();

    for (square, piece) in board.board.iter().enumerate() {

        let row = square / 8;
        let col = square % 8;

        match piece {

            Piece::Empty => {},

            Piece::Pawn{colour} if *colour == side_to_move => {

                if side_to_move == Colour::Black {

                    try_add_move(&mut moves, board, square, 1, 0, MoveType::NonCapture);
                    try_add_move(&mut moves, board, square, 1, -1, MoveType::Capture);
                    try_add_move(&mut moves, board, square, 1, 1, MoveType::Capture);

                    if row == 1 && board.board[square + 8] == Piece::Empty {
                        try_add_move(&mut moves, board, square, 2, 0, MoveType::NonCapture);
                    }

                    match board.en_passant_chance {
                        Some(en_passant_square) => {
                            let en_passant_row = en_passant_square / 8;
                            let en_passant_col = en_passant_square % 8;
                            if en_passant_row == row && (en_passant_col as isize - col as isize).abs() == 1 {
                                moves.push(Move::new(&board, square, en_passant_square + 8));
                            }
                        }
                        None => {}
                    }

                } else {

                    try_add_move(&mut moves, board, square, -1, 0, MoveType::NonCapture);
                    try_add_move(&mut moves, board, square, -1, -1, MoveType::Capture);
                    try_add_move(&mut moves, board, square, -1, 1, MoveType::Capture);

                    if row == 6 && board.board[square - 8] == Piece::Empty {
                        try_add_move(&mut moves, board, square, -2, 0, MoveType::NonCapture);
                    }

                    match board.en_passant_chance {
                        Some(en_passant_square) => {
                            let en_passant_row = en_passant_square / 8;
                            let en_passant_col = en_passant_square % 8;
                            if en_passant_row == row && (en_passant_col as isize - col as isize).abs() == 1 {
                                moves.push(Move::new(&board, square, en_passant_square - 8));
                            }
                        }
                        None => {}
                    }
                    
                }
            },

            Piece::Knight{colour} if *colour == side_to_move => {
                try_add_move(&mut moves, board, square, -2, -1, MoveType::Move);
                try_add_move(&mut moves, board, square, -2, 1, MoveType::Move);
                try_add_move(&mut moves, board, square, 2, -1, MoveType::Move);
                try_add_move(&mut moves, board, square, 2, 1, MoveType::Move);
                try_add_move(&mut moves, board, square, -1, -2, MoveType::Move);
                try_add_move(&mut moves, board, square, -1, 2, MoveType::Move);
                try_add_move(&mut moves, board, square, 1, 2, MoveType::Move);
                try_add_move(&mut moves, board, square, 1, -2, MoveType::Move);
            },

            Piece::Bishop{colour} if *colour == side_to_move => {
                add_sliding_moves(&mut moves, board, square, false, true);
            },

            Piece::Rook{colour} if *colour == side_to_move => {
                add_sliding_moves(&mut moves, board, square, true, false);
            },

            Piece::Queen{colour} if *colour == side_to_move => {
                add_sliding_moves(&mut moves, board, square, true, true);
            },

            Piece::King{colour} if *colour == side_to_move => {

                try_add_move(&mut moves, board, square, -1, -1, MoveType::Move);
                try_add_move(&mut moves, board, square, -1, 0, MoveType::Move);
                try_add_move(&mut moves, board, square, -1, 1, MoveType::Move);
                try_add_move(&mut moves, board, square, 0, -1, MoveType::Move);
                try_add_move(&mut moves, board, square, 0, 1, MoveType::Move);
                try_add_move(&mut moves, board, square, 1, -1, MoveType::Move);
                try_add_move(&mut moves, board, square, 1, 0, MoveType::Move);
                try_add_move(&mut moves, board, square, 1, 1, MoveType::Move);

                match colour {
                    Colour::White => {
                        if board.castling_rights.1 && board.board[57] == Piece::Empty && board.board[58] == Piece::Empty && board.board[59] == Piece::Empty {
                            moves.push(Move::new(&board, square, 58));
                        }
                        if board.castling_rights.0 && board.board[61] == Piece::Empty && board.board[62] == Piece::Empty {
                            moves.push(Move::new(&board, square, 62));
                        }
                    },
                    Colour::Black => {
                        if board.castling_rights.3 && board.board[1] == Piece::Empty && board.board[2] == Piece::Empty && board.board[3] == Piece::Empty {
                            moves.push(Move::new(&board, square, 2));
                        }
                        if board.castling_rights.2 && board.board[5] == Piece::Empty && board.board[6] == Piece::Empty {
                            moves.push(Move::new(&board, square, 6));
                        }
                    },
                }

            },
            
            _ => {},

        }
    }

    if ignore_check {
        return moves
    }

    let mut legal_moves: Vec<Move> = Vec::new();

    for pseudo_legal_move in moves { // beyond terrible
        board.make_move(&pseudo_legal_move);
        let mut move_is_legal = true;
        for response_move in get_possible_moves(board, true) {
            if response_move.end_square == board.black_king || response_move.end_square == board.white_king {
                move_is_legal = false;
                break;
            }
        }
        if move_is_legal {
            legal_moves.push(pseudo_legal_move);
        }
        board.undo_move();
    }

    return legal_moves;

}

pub fn get_num_moves(board: &mut Board, depth: usize) -> usize {

    let possible_moves = get_possible_moves(board, false);
    
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