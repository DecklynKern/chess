#![allow(non_upper_case_globals)]
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

        let move_type = if moved_piece.is_pawn() && ((start_square as isize - end_square as isize).abs() - 12).abs() == 1 && replaced_piece == Empty  {
            SpecialMoveType::EnPassant
        } else if moved_piece.is_king() && (start_square as isize - end_square as isize).abs() == 2 {
            SpecialMoveType::Castle
        } else {
            SpecialMoveType::Normal
        };

        Move {
            start_square: start_square,
            end_square: end_square,
            moved_piece: moved_piece,
            replaced_piece: replaced_piece,
            old_en_passant_chance: board.en_passant_chance,
            old_castling_rights: board.castling_rights,
            move_type: move_type
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
            if self.moved_piece.is_pawn() {
                String::from("")
            } else {
                self.moved_piece.to_char().to_string()
            },
            if same_dest_moves.is_empty() {
                String::from("")
            } else {
                index_to_an(self.start_square)
            },
            if self.replaced_piece != Empty || self.move_type == SpecialMoveType::EnPassant {"x"} else {""},
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

    if end_piece == Border {
        return AddResult::Fail;
    }

    let end_piece_empty = end_piece == Empty;
    let same_colour = board.get_piece_abs(start_square).same_colour(&end_piece);

    if (!end_piece_empty && (same_colour || move_type == MoveType::NonCapture)) ||
        (move_type == MoveType::Capture && (end_piece_empty || same_colour)) {
        return AddResult::Fail;
    }

    let new_move = Move::new(&board, start_square, end_square);
    let result = if new_move.replaced_piece != Empty {AddResult::Capture} else {AddResult::Move};

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

pub fn get_possible_moves(board: &Board) -> Vec<Move> {

    let side_to_move = board.side_to_move;
    let mut moves: Vec<Move> = Vec::new();

    let king_square: usize;
    let knight: Piece;
    let bishop: Piece;
    let rook: Piece;
    let queen: Piece;
    let own_king: Piece;

    if side_to_move == White {

        for &pawn in &board.piece_positions[WhitePawn as usize] {

            try_add_move(&mut moves, board, pawn, -12, MoveType::NonCapture);
            try_add_move(&mut moves, board, pawn, -11, MoveType::Capture);
            try_add_move(&mut moves, board, pawn, -13, MoveType::Capture);

            if pawn > 95 && board.get_piece_abs(pawn - 12) == Empty {
                try_add_move(&mut moves, board, pawn, -24, MoveType::NonCapture);
            }

            match board.en_passant_chance {
                Some(en_passant_square) => if (en_passant_square as isize - pawn as isize + 12).abs() == 1 {moves.push(Move::new(&board, pawn, en_passant_square))},
                None => {}
            }

        }

        knight = WhiteKnight;
        bishop = WhiteBishop;
        rook = WhiteRook;
        queen = WhiteQueen;

        king_square = board.white_king;
        own_king = WhiteKing;

        if board.castling_rights.1 && board.get_piece_abs(111) == Empty &&
        board.get_piece_abs(112) == Empty && board.get_piece_abs(113) == Empty && !is_attacking_square(113, &board, Black) {
            moves.push(Move::new(&board, king_square, 112));
        }
        if board.castling_rights.0 && board.get_piece_abs(115) == Empty &&
        board.get_piece_abs(116) == Empty && !is_attacking_square(115, &board, Black) {
            moves.push(Move::new(&board, king_square, 116));
        }

    } else {

        for &pawn in &board.piece_positions[BlackPawn as usize] {

            try_add_move(&mut moves, board, pawn, 12, MoveType::NonCapture);
            try_add_move(&mut moves, board, pawn, 11, MoveType::Capture);
            try_add_move(&mut moves, board, pawn, 13, MoveType::Capture);

            if pawn < 46 && board.get_piece_abs(pawn + 12) == Empty {
                try_add_move(&mut moves, board, pawn, 24, MoveType::NonCapture);
            }

            match board.en_passant_chance {
                Some(en_passant_square) => if (en_passant_square as isize - pawn as isize - 12).abs() == 1 {moves.push(Move::new(&board, pawn, en_passant_square))},
                None => {}
            }

        }

        knight = BlackKnight;
        bishop = BlackBishop;
        rook = BlackRook;
        queen = BlackQueen;

        king_square = board.black_king;
        own_king = BlackKing;

        if board.castling_rights.3 && board.get_piece_abs(27) == Empty &&
        board.get_piece_abs(28) == Empty && board.get_piece_abs(29) == Empty && !is_attacking_square(29, &board, White) {
            moves.push(Move::new(&board, king_square, 28));
        }
        if board.castling_rights.2 && board.get_piece_abs(31) == Empty &&
        board.get_piece_abs(32) == Empty && !is_attacking_square(31, &board, White) {
            moves.push(Move::new(&board, king_square, 32));
        }

    }

    for &knight in &board.piece_positions[knight as usize] {
        for offset in KNIGHT_OFFSETS {
            try_add_move(&mut moves, board, knight, offset, MoveType::Move);
        }
    }

    for &bishop in &board.piece_positions[bishop as usize] {
        add_sliding_moves(&mut moves, board, bishop, false, true);
    }

    for &rook in &board.piece_positions[rook as usize] {
        add_sliding_moves(&mut moves, board, rook, true, false);
    }

    for &queen in &board.piece_positions[queen as usize] {
        add_sliding_moves(&mut moves, board, queen, true, true);
    }

    for offset in KING_OFFSETS {
        try_add_move(&mut moves, board, king_square, offset, MoveType::Move);
    }

    let mut legal_moves: Vec<Move> = Vec::new();

    let king_attackers = get_king_attackers(&board, side_to_move);
    let pinned_pieces = get_pinned_pieces(&board, side_to_move);
    let attacked_squares = get_attacked_squares_surrounding_king(&board, side_to_move);

    if king_attackers.is_empty() { // can't move king into check or move pinned pieces

        for pseudo_legal_move in moves {
            
            if pseudo_legal_move.moved_piece == own_king && attacked_squares & 1 << pseudo_legal_move.end_square != 0 {
                continue;
            }

            let mut is_pinned = false;

            for (pinned_square, safe_squares) in &pinned_pieces {
                if *pinned_square == pseudo_legal_move.start_square && safe_squares & 1 << pseudo_legal_move.end_square == 0 {
                    is_pinned = true;
                    break;
                }
            }

            if !is_pinned {
                legal_moves.push(pseudo_legal_move);
            }
        }

    } else if king_attackers.len() == 1 { // can only move king out of the way or block, no castling though
        
        let block_squares = &king_attackers[0];

        for pseudo_legal_move in moves {

            if pseudo_legal_move.move_type == SpecialMoveType::Castle {
                continue;
            }

            if pseudo_legal_move.moved_piece == own_king && attacked_squares & 1 << pseudo_legal_move.end_square == 0 ||
            pseudo_legal_move.moved_piece != own_king && block_squares & 1 << pseudo_legal_move.end_square != 0 {
                
                let mut is_pinned = false;

                for (pinned_square, safe_squares) in &pinned_pieces {
                    if *pinned_square == pseudo_legal_move.start_square && safe_squares & 1 << pseudo_legal_move.end_square == 0 {
                        is_pinned = true;
                        break;
                    }
                }

                if !is_pinned {
                    legal_moves.push(pseudo_legal_move);
                }
            }
        }

    } else { // can only move king, no castling though

        for pseudo_legal_move in moves {

            if pseudo_legal_move.moved_piece == own_king && attacked_squares & 1 << pseudo_legal_move.end_square == 0
            && pseudo_legal_move.move_type != SpecialMoveType::Castle {
                legal_moves.push(pseudo_legal_move);
            }
        }
    }

    return legal_moves;

}

pub fn get_king_attackers(board: &Board, colour: Colour) -> Vec<u128> {

    let mut attackers = Vec::new();

    let king_square = match colour {
        White => board.white_king,
        Black => board.black_king
    } as isize;

    let opp_colour = colour.opposite() as u8;
    let opp_knight = KNIGHT | opp_colour;

    for offset in KNIGHT_OFFSETS {
        let test_square = (king_square + offset) as usize;
        if board.get_piece_abs(test_square) as u8 == opp_knight {
            attackers.push(1 << test_square);
        }
    }

    // assumes no back rank pawns
    if colour == White && king_square > 48 {

        let test_square1 = (king_square - 11) as usize;
        let test_square2 = (king_square - 13) as usize;

        if board.get_piece_abs(test_square1) == BlackPawn {
            attackers.push(1 << test_square1);

        } else if board.get_piece_abs(test_square2) == BlackPawn {
            attackers.push(1 << test_square2);
        }

    } else if colour == Black && king_square < 96 {

        let test_square1 = (king_square + 11) as usize;
        let test_square2 = (king_square + 13) as usize;

        if board.get_piece_abs(test_square1) == WhitePawn {
            attackers.push(1 << test_square1);

        } else if board.get_piece_abs(test_square2) == WhitePawn {
            attackers.push(1 << test_square2);
        }
    }

    let opp_bishop = BISHOP | opp_colour;
    let opp_queen = QUEEN | opp_colour;

    for dir in DIAGONAL_OFFSETS {
        
        let mut test_square = king_square;
        let mut safe_squares = 0u128;
        
        loop {
            
            test_square += dir;

            let piece = board.get_piece_abs(test_square as usize);

            if piece == Empty {
                safe_squares |= 1 << test_square;
                continue;
            }

            if piece as u8 == opp_bishop || piece as u8 == opp_queen {
                safe_squares |= 1 << test_square;
                attackers.push(safe_squares);
                break;
            }

            break;

        }
    }
    
    let opp_rook = ROOK | opp_colour;

    for dir in ORTHOGONAL_OFFSETS {
        
        let mut test_square = king_square;
        let mut safe_squares = 0u128;
        
        loop {
            
            test_square += dir;

            let piece = board.get_piece_abs(test_square as usize);

            if piece == Empty {
                safe_squares |= 1 << test_square;
                continue;
            }

            if piece as u8 == opp_rook || piece as u8 == opp_queen {
                safe_squares |= 1 << test_square;
                attackers.push(safe_squares);
                break;
            }

            break;
            
        }
    }
    
    let opp_king = KING | opp_colour;

    for offset in KING_OFFSETS {
        let test_square = (king_square + offset) as usize;
        if board.get_piece_abs(test_square) as u8 == opp_king {
            attackers.push(1 << test_square);
        }
    }

    return attackers;

}

pub fn get_pinned_pieces(board: &Board, colour: Colour) -> Vec<(usize, u128)> {

    let mut pinned_pieces = Vec::new();

    let king_square = match colour {
        White => board.white_king,
        Black => board.black_king
    } as isize;

    let opp_colour = colour.opposite() as u8;

    let opp_bishop = BISHOP | opp_colour;
    let opp_queen = QUEEN | opp_colour;

    for dir in DIAGONAL_OFFSETS {
        
        let mut test_square = king_square;
        let mut pinned_square = 0;
        let mut safe_squares = 0u128;
        
        loop {
            
            test_square += dir;

            let piece = board.get_piece_abs(test_square as usize);

            if piece == Empty {
                safe_squares |= 1 << test_square;
                continue;

            } else if piece == Border {
                break;
            }

            if piece.is_colour(colour) {
                if pinned_square != 0 {
                    break;

                } else {
                    pinned_square = test_square as usize;
                    continue;
                }
            }

            if pinned_square != 0 && (piece as u8 == opp_bishop || piece as u8 == opp_queen) {
                safe_squares |= 1 << test_square;
                pinned_pieces.push((pinned_square, safe_squares));
                break;
            }

            break;

        }
    }
    
    let opp_rook = ROOK | opp_colour;

    for dir in ORTHOGONAL_OFFSETS {
        
        let mut test_square = king_square;
        let mut pinned_square = 0;
        let mut safe_squares = 0u128;
        
        loop {
            
            test_square += dir;

            let piece = board.get_piece_abs(test_square as usize);

            if piece == Empty {
                safe_squares |= 1 << test_square;
                continue;

            } else if piece == Border {
                break;
            }

            if piece.is_colour(colour) {
                if pinned_square != 0 {
                    break;

                } else {
                    pinned_square = test_square as usize;
                    continue;
                }
            }

            if pinned_square != 0 && (piece as u8 == opp_rook || piece as u8  == opp_queen) {
                safe_squares |= 1 << test_square;
                pinned_pieces.push((pinned_square, safe_squares));
                break;
            }

            break;
            
        }
    }

    return pinned_pieces;

}

pub fn is_attacking_square(square: usize, board: &Board, colour: Colour) -> bool {

    let sq = square as isize;

    // println!("pawn");

    // assumes no back rank pawns
    if colour == White {

        if board.get_piece_abs((sq + 11) as usize) == WhitePawn {
            return true;

        } else if board.get_piece_abs((sq + 13) as usize) == WhitePawn {
            return true;
        }

    } else if colour == Black {

        if board.get_piece_abs((sq - 11) as usize) == BlackPawn {
            return true;

        } else if board.get_piece_abs((sq - 13) as usize) == BlackPawn {
            return true;
        }
    }

    // println!("knight");

    let colour_num = colour as u8;
    let knight = KNIGHT | colour_num;

    for offset in KNIGHT_OFFSETS {
        if board.get_piece_abs((sq + offset) as usize) as u8 == knight {
            return true;
        }
    }
    
    // println!("diagonal");

    let bishop = BISHOP | colour_num;
    let queen = QUEEN | colour_num;

    for dir in DIAGONAL_OFFSETS {
        
        let mut test_square = sq;
        
        loop {
            
            test_square += dir;

            let piece = board.get_piece_abs(test_square as usize);

            if piece == Empty || piece.is_king() {
                continue;
            }

            if piece as u8 == bishop || piece as u8 == queen {
                return true;
            }

            break;

        }
    }
    
    // println!("orthogonal");
    
    let rook = ROOK | colour_num;

    for dir in ORTHOGONAL_OFFSETS {
        
        let mut test_square = sq;
        
        loop {
            
            test_square += dir;

            let piece = board.get_piece_abs(test_square as usize);

            if piece == Empty || piece.is_king() {
                continue;
            }

            if piece as u8 == rook || piece as u8 == queen {
                return true;
            }

            break;
            
        }
    }
    
    // println!("king");
    
    let king = KING | colour_num;

    for offset in KING_OFFSETS {
        if board.get_piece_abs((sq + offset) as usize) as u8 == king {
            return true;
        }
    }

    return false;

}

pub fn get_attacked_squares_surrounding_king(board: &Board, colour: Colour) -> u128 { // bitboard :)

    let mut attacked_squares = 0;

    let king = match colour {
        White => board.white_king,
        Black => board.black_king
    } as isize;

    for offset in KING_OFFSETS {
        let test_square = (king + offset) as usize;
        if test_square > 127 {
            break;
        }
        if board.get_piece_abs(test_square) == Border || is_attacking_square(test_square, board, colour.opposite()) {
            attacked_squares |= 1 << test_square;
        }
    }

    match colour {
        White => {
            if board.castling_rights.0 && is_attacking_square(112, board, Black) {
                attacked_squares |= 1 << 112;
            }
            if board.castling_rights.1 && is_attacking_square(116, board, Black) {
                attacked_squares |= 1 << 116;
            }
        },
        Black => {
            if board.castling_rights.2 && is_attacking_square(28, board, White) {
                attacked_squares |= 1 << 28;
            }
            if board.castling_rights.3 && is_attacking_square(32, board, White) {
                attacked_squares |= 1 << 32;
            }
        }
    }

    return attacked_squares;

}

pub fn get_num_moves(board: &mut Board, depth: usize) -> usize {

    if depth == 0 {
        return 1;
    }

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