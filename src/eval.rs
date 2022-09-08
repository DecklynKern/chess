use crate::board::*;

#[derive(Clone, Copy, PartialEq)]
pub enum SpecialMoveType {
    Normal,
    EnPassant,
    Castle
}

#[derive(Clone, Copy)]
pub struct Move {
    pub start_square: usize,
    pub end_square: usize,
    pub moved_piece: Piece,
    pub replaced_piece: Piece,
    pub old_en_passant_chance: Option<usize>,
    pub special_move_type: SpecialMoveType
}

impl Move {

    fn new(board: &Board, start_square: usize, end_square: usize, special_move_type: SpecialMoveType) -> Move {
        Move {
            start_square: start_square,
            end_square: end_square,
            moved_piece: board.board[start_square],
            replaced_piece: board.board[end_square],
            old_en_passant_chance: board.en_passant_chance,
            special_move_type: special_move_type
        }
    }

    fn index_to_an(idx: usize) -> String {

        let rank = (8 - idx / 8);
        let file = String::from("abcdefgh").chars().nth(idx % 8).unwrap();
    
        return format!("{}{}", file, rank);

    }

    pub fn to_an(&self, possible_moves: &Vec<Move>) -> String {

        let mut same_dest_moves: Vec<&Move> = Vec::new();

        for possible_move in possible_moves {
            if possible_move.end_square == self.end_square && possible_move.start_square != self.start_square && possible_move.moved_piece == self.moved_piece {
                same_dest_moves.push(possible_move);
            }
        }

        return format!("{}{}{}{}", 
        
            match self.moved_piece {
                Piece::Pawn {colour: _} => String::from(""),
                piece => piece.to_char().to_string(),
            },

            if same_dest_moves.is_empty() {
                String::from("")
            } else {
                Move::index_to_an(self.start_square)
            },

            if self.replaced_piece != Piece::Empty || self.special_move_type == SpecialMoveType::EnPassant {"x"} else {""},

            Move::index_to_an(self.end_square)

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

    let new_move = Move::new(&board, start_square, end_square, SpecialMoveType::Normal);
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
                                moves.push(Move::new(&board, square, en_passant_square + 8, SpecialMoveType::EnPassant));
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
                                moves.push(Move::new(&board, square, en_passant_square - 8, SpecialMoveType::EnPassant));
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