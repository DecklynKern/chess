use crate::simulator::piece::*;
use crate::simulator::board::*;
use crate::simulator::chess_util::*;

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
    pub old_castling_rights : (bool, bool, bool, bool),
    pub move_type: SpecialMoveType
}

impl Move {

    pub fn new(board: &Board, start_square: usize, end_square: usize) -> Move {

        let moved_piece = board.get_piece_abs(start_square);
        let replaced_piece = board.get_piece_abs(end_square);

        Move {
            start_square: start_square,
            end_square: end_square,
            moved_piece: moved_piece,
            replaced_piece: replaced_piece,
            old_en_passant_chance: board.en_passant_chance,
            old_castling_rights: board.castling_rights,
            move_type: match moved_piece {
                Piece::Pawn{colour: _} if ((start_square as isize - end_square as isize).abs() - 12).abs() == 1 && replaced_piece == EMPTY => SpecialMoveType::EnPassant,
                Piece::King{colour: _} if (start_square as isize - end_square as isize).abs() == 2 => SpecialMoveType::Castle,
                _ => SpecialMoveType::Normal
            }
        }
    }

    pub fn to_long_an(&self) -> String {
        return format!("{}{}", index_to_long_an(self.start_square), index_to_long_an(self.end_square));
    }

    pub fn to_an(&self, possible_moves: &Vec<Move>) -> String {

        let mut same_dest_moves: Vec<&Move> = Vec::new();

        for possible_move in possible_moves {
            if possible_move.moved_piece == self.moved_piece && possible_move.end_square == self.end_square && possible_move.start_square != self.start_square {
                same_dest_moves.push(possible_move);
            }

        }

        if self.move_type == SpecialMoveType::Castle {
            return String::from(match self.end_square {
                28 => "o-o-o",
                32 => "o-o",
                112 => "O-O-O",
                116 => "O-O",
                _ => unreachable!()
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
                index_to_an(self.start_square)
            },
            if self.replaced_piece != EMPTY || self.move_type == SpecialMoveType::EnPassant {"x"} else {""},
            index_to_an(self.end_square)
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
enum MoveType {
    NonCapture,
    Capture,
    Move
}

const KNIGHT_OFFSETS: [isize; 8] = [-25, -23, -14, -10, 25, 23, 14, 10];
const KING_OFFSETS: [isize; 8] = [-13, -12, -11, -1, 13, 12, 11, 1];
const ORTHOGONAL_OFFSETS: [isize; 4] = [-12, -1, 12, 1];
const DIAGONAL_OFFSETS: [isize; 4] = [-13, -11, 13, 11];

fn try_add_move(moves: &mut Vec<Move>, board: &Board, start_square: usize, offset: isize, move_type: MoveType) -> AddResult {

    let end_square = (start_square as isize + offset) as usize;
    let end_piece = board.get_piece_abs(end_square);

    if end_piece == BORDER {
        return AddResult::Fail;
    }

    let end_piece_empty = end_piece == EMPTY;

    if (!end_piece_empty && board.get_piece_abs(start_square).get_colour() == end_piece.get_colour()) ||
        (move_type == MoveType::Capture && (end_piece_empty || board.get_piece_abs(start_square).get_colour() == end_piece.get_colour())) ||
        (move_type == MoveType::NonCapture && !end_piece_empty) {
        return AddResult::Fail;
    }

    let new_move = Move::new(&board, start_square, end_square);
    let result = if new_move.replaced_piece != EMPTY {AddResult::Capture} else {AddResult::Move};

    moves.push(new_move);

    return result;

}

fn add_sliding_moves(moves: &mut Vec<Move>, board: &Board, start_square: usize, orthogonal: bool, diagonal: bool) {

    if orthogonal {

        for offset in ORTHOGONAL_OFFSETS {

            let mut total_offset = 0;
    
            loop {
    
                total_offset += offset;
    
                if try_add_move(moves, board, start_square, total_offset, MoveType::Move) != AddResult::Move {
                    break;
                }
    
            }
        }
    }

    if diagonal {

        for offset in DIAGONAL_OFFSETS {

            let mut total_offset = 0;
    
            loop {
    
                total_offset += offset;
    
                if try_add_move(moves, board, start_square, total_offset, MoveType::Move) != AddResult::Move {
                    break;
                }
    
            }
        }
    }
}

pub fn get_possible_moves(board: &mut Board) -> Vec<Move> {

    let side_to_move = board.side_to_move;
    let mut moves: Vec<Move> = Vec::new();

    for square in VALID_SQUARES {

        let (row, _) = Board::pos_to_row_col(square);

        match &board.get_piece_abs(square) {

            Piece::Empty => {},

            Piece::Pawn{colour} if *colour == side_to_move => {

                if side_to_move == Colour::White {

                    try_add_move(&mut moves, board, square, -12, MoveType::NonCapture);
                    try_add_move(&mut moves, board, square, -11, MoveType::Capture);
                    try_add_move(&mut moves, board, square, -13, MoveType::Capture);

                    if row == 6 && board.get_piece_abs(square - 12) == EMPTY {
                        try_add_move(&mut moves, board, square, -24, MoveType::NonCapture);
                    }

                    match board.en_passant_chance {
                        Some(en_passant_square) => {
                            if (en_passant_square as isize - square as isize).abs() == 1 {
                                moves.push(Move::new(&board, square, en_passant_square - 12));
                            }
                        }
                        None => {}
                    }

                } else {

                    try_add_move(&mut moves, board, square, 12, MoveType::NonCapture);
                    try_add_move(&mut moves, board, square, 11, MoveType::Capture);
                    try_add_move(&mut moves, board, square, 13, MoveType::Capture);

                    if row == 1 && board.get_piece_abs(square + 12) == EMPTY {
                        try_add_move(&mut moves, board, square, 24, MoveType::NonCapture);
                    }

                    match board.en_passant_chance {
                        Some(en_passant_square) => {
                            if (en_passant_square as isize - square as isize).abs() == 1 {
                                moves.push(Move::new(&board, square, en_passant_square + 12));
                            }
                        }
                        None => {}
                    }
                    
                }
            },

            Piece::Knight{colour} if *colour == side_to_move => {
                for offset in KNIGHT_OFFSETS {
                    try_add_move(&mut moves, board, square, offset, MoveType::Move);
                }
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

                for offset in KING_OFFSETS {
                    try_add_move(&mut moves, board, square, offset, MoveType::Move);
                }

                match colour {
                    Colour::White => {
                        if board.castling_rights.1 && board.get_piece_abs(111) == EMPTY && board.get_piece_abs(112) == EMPTY && board.get_piece_abs(113) == EMPTY {
                            moves.push(Move::new(&board, square, 112));
                        }
                        if board.castling_rights.0 && board.get_piece_abs(115) == EMPTY && board.get_piece_abs(116) == EMPTY {
                            moves.push(Move::new(&board, square, 116));
                        }
                    },
                    Colour::Black => {
                        if board.castling_rights.3 && board.get_piece_abs(27) == EMPTY && board.get_piece_abs(28) == EMPTY && board.get_piece_abs(29) == EMPTY {
                            moves.push(Move::new(&board, square, 28));
                        }
                        if board.castling_rights.2 && board.get_piece_abs(31) == EMPTY && board.get_piece_abs(32) == EMPTY {
                            moves.push(Move::new(&board, square, 32));
                        }
                    },
                }
            },
            _ => {}
        }
    }

    let mut legal_moves: Vec<Move> = Vec::new();

    for pseudo_legal_move in moves { // a bit better now, still needs work
        board.make_move(&pseudo_legal_move);
        if !is_in_check(&board) {
            legal_moves.push(pseudo_legal_move);
        } //else {println!("{}", board.to_fen())}
        board.undo_move();
    }

    return legal_moves;

}

// assumes a move has just been played
pub fn is_in_check(board: &Board) -> bool {

    let current_colour = board.side_to_move.opposite();

    let king_square = match current_colour {
        Colour::White => board.white_king,
        Colour::Black => board.black_king
    } as isize;

    let opp_colour = board.side_to_move;
    let opp_knight = Piece::Knight{colour: opp_colour};

    for offset in KNIGHT_OFFSETS {
        if board.get_piece_abs((king_square + offset) as usize) == opp_knight {
            return true;
        }
    }

    // assumes no back rank pawns
    if current_colour == Colour::White {
        if king_square > 48 && (board.get_piece_abs((king_square - 11) as usize) == BLACK_PAWN || board.get_piece_abs((king_square - 13) as usize) == BLACK_PAWN) {
            return true;
        }
    } else if king_square < 96 && (board.get_piece_abs((king_square + 11) as usize) == WHITE_PAWN || board.get_piece_abs((king_square + 13) as usize) == WHITE_PAWN) {
        return true;
    }

    let opp_bishop = Piece::Bishop{colour: opp_colour};
    let opp_queen = Piece::Queen{colour: opp_colour};

    for dir in DIAGONAL_OFFSETS {
        
        let mut test_square = king_square;
        
        loop {
            
            test_square += dir;

            let piece = board.get_piece_abs(test_square as usize);

            if piece == EMPTY {
                continue;
            }

            if piece == opp_bishop || piece == opp_queen {
                return true;
            }

            break;

        }
    }
    
    let opp_rook = Piece::Rook{colour: opp_colour};

    for dir in ORTHOGONAL_OFFSETS {
        
        let mut test_square = king_square;
        
        loop {
            
            test_square += dir;

            let piece = board.get_piece_abs(test_square as usize);

            if piece == EMPTY {
                continue;
            }

            if piece == opp_rook || piece == opp_queen {
                return true;
            }

            break;
            
        }
    }
    
    let opp_king = Piece::King{colour: opp_colour};

    for offset in KING_OFFSETS {
        if board.get_piece_abs((king_square + offset) as usize) == opp_king {
            return true;
        }
    }

    return false;

}

pub fn get_num_moves(board: &mut Board, depth: usize) -> usize {

    let possible_moves = get_possible_moves(board);
    
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