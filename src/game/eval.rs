#![allow(non_upper_case_globals)]
use super::chess_util::*;
use super::piece::*;
use super::board::*;
use super::r#move::*;

const KNIGHT_OFFSETS: [isize; 8] = [-25, -23, -14, -10, 25, 23, 14, 10];
const KING_OFFSETS: [isize; 8] = [-13, -12, -11, -1, 1, 11, 12, 13];
const ORTHOGONAL_OFFSETS: [isize; 4] = [-12, -1, 12, 1];
const DIAGONAL_OFFSETS: [isize; 4] = [-13, -11, 13, 11];

pub fn is_back_rank(colour: Colour, square: usize) -> bool {
    (colour == White && square >= A1) || (colour == Black && square <= H8)
}

pub fn is_back_two_ranks(colour: Colour, square: usize) -> bool {
    (colour == White && square >= A2) || (colour == Black && square <= H7)
}

fn try_add_move(moves: &mut Vec<Move>, board: &Board, start_square: usize, offset: isize) -> bool {

    let end_square = (start_square as isize + offset) as usize;
    let end_piece = board.get_piece_abs(end_square);

    if end_piece == Border {
        return false;
    }

    let end_piece_empty = end_piece == Empty;
    let same_colour = board.get_piece_abs(start_square).same_colour(end_piece);

    if !end_piece_empty && same_colour {
        return false;
    }

    let new_move = Move::new(board, start_square, end_square);
    let is_empty = new_move.replaced_piece == Empty;
    moves.push(new_move);

    return is_empty;

}

fn add_pawn_moves(moves: &mut Vec<Move>, board:&Board, start_square: usize, end_square: usize, colour: u8, is_promo: bool) {
    if is_promo {
        moves.push(Move::new_promotion(board, start_square, end_square, Piece::from_num(colour | QUEEN)));
        moves.push(Move::new_promotion(board, start_square, end_square, Piece::from_num(colour | ROOK)));
        moves.push(Move::new_promotion(board, start_square, end_square, Piece::from_num(colour | BISHOP)));
        moves.push(Move::new_promotion(board, start_square, end_square, Piece::from_num(colour | KNIGHT)));

    } else {
        moves.push(Move::new(board, start_square, end_square));
    }
}

fn gen_valid_pawn_moves(moves: &mut Vec<Move>, board:&Board, start_square: usize, colour: Colour) {

    let forward_square = colour.offset_index(start_square);

    let rank = start_square % 12;
    let is_promo = is_back_rank(colour.opposite(), forward_square);

    if rank != 2 {

        let test_square = forward_square - 1;
        let piece_at = board.get_piece_abs(test_square);

        if board.get_piece_abs(test_square) != Piece::Empty && piece_at.get_colour() != colour {
            add_pawn_moves(moves, board, start_square, test_square, colour as u8, is_promo);
        }
    }

    if rank != 9 {

        let test_square = forward_square + 1;
        let piece_at = board.get_piece_abs(test_square);

        if board.get_piece_abs(test_square) != Piece::Empty && piece_at.get_colour() != colour {
            add_pawn_moves(moves, board, start_square, test_square, colour as u8, is_promo);
        }
    }

    if board.get_piece_abs(forward_square) == Piece::Empty {

        add_pawn_moves(moves, board, start_square, forward_square, colour as u8, is_promo);

        let double_move_square = colour.offset_index(forward_square);

        if board.get_piece_abs(double_move_square) == Empty && is_back_two_ranks(colour, start_square) {
            moves.push(Move::new_pawn_double(board, start_square, double_move_square));
        }

    }

    if let Some(en_passant_square) = board.en_passant_chance {
        if (en_passant_square as isize - colour.offset_index(start_square) as isize).abs() == 1 {
            moves.push(Move::new_en_passant(board, start_square, en_passant_square))
        }
    }

}

fn add_sliding_moves(moves: &mut Vec<Move>, board: &Board, start_square: usize, orthogonal: bool, diagonal: bool) {

    if orthogonal {

        for offset in ORTHOGONAL_OFFSETS {

            let mut total_offset = offset;
    
            while try_add_move(moves, board, start_square, total_offset) {
                total_offset += offset;
            }
        }
    }

    if diagonal {

        for offset in DIAGONAL_OFFSETS {

            let mut total_offset = offset;
    
            while try_add_move(moves, board, start_square, total_offset) {
                total_offset += offset;
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
            gen_valid_pawn_moves(&mut moves, board, pawn, White);
        }

        knight = WhiteKnight;
        bishop = WhiteBishop;
        rook = WhiteRook;
        queen = WhiteQueen;

        king_square = board.white_king;
        own_king = WhiteKing;

        if board.castling_rights & WHITE_KINGSIDE != NO_CASTLING_RIGHTS && board.get_piece_abs(F1) == Empty && board.get_piece_abs(G1) == Empty &&
        !is_attacking_square(F1, board, Black) && !is_attacking_square(G1, board, Black) {
            moves.push(Move::new_castle(board, king_square, G1));
        }
        if board.castling_rights & WHITE_QUEENSIDE != NO_CASTLING_RIGHTS && board.get_piece_abs(B1) == Empty && board.get_piece_abs(C1) == Empty &&
        board.get_piece_abs(D1) == Empty && !is_attacking_square(C1, board, Black) && !is_attacking_square(D1, board, Black) {
            moves.push(Move::new_castle(board, king_square, C1));
        }

    } else {

        for &pawn in &board.piece_positions[BlackPawn as usize] {
            gen_valid_pawn_moves(&mut moves, board, pawn, Black);
        }

        knight = BlackKnight;
        bishop = BlackBishop;
        rook = BlackRook;
        queen = BlackQueen;

        king_square = board.black_king;
        own_king = BlackKing;

        if board.castling_rights & BLACK_KINGSIDE != NO_CASTLING_RIGHTS && board.get_piece_abs(F8) == Empty && board.get_piece_abs(G8) == Empty &&
        !is_attacking_square(F8, board, White) && !is_attacking_square(G8, board, White) {
            moves.push(Move::new_castle(board, king_square, G8));
        }
        if board.castling_rights & BLACK_QUEENSIDE != NO_CASTLING_RIGHTS && board.get_piece_abs(B8) == Empty && board.get_piece_abs(C8) == Empty &&
        board.get_piece_abs(D8) == Empty && !is_attacking_square(C8, board, White) && !is_attacking_square(D8, board, White) {
            moves.push(Move::new_castle(board, king_square, C8));
        }

    }

    for &knight in &board.piece_positions[knight as usize] {
        for offset in KNIGHT_OFFSETS {
            try_add_move(&mut moves, board, knight, offset);
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
        try_add_move(&mut moves, board, king_square, offset);
    }

    let mut legal_moves: Vec<Move> = Vec::new();

    let king_attackers = get_king_attackers(board, side_to_move);
    let pinned_pieces = get_pinned_pieces(board, side_to_move);
    let attacked_squares = get_attacked_squares_surrounding_king(board, side_to_move);

    // can't move king into check or move pinned pieces
    if king_attackers.is_empty() {

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

    // can only move king out of the way or block, no castling though
    } else if king_attackers.len() == 1 {
        
        let block_squares = &king_attackers[0];

        for pseudo_legal_move in moves {

            if pseudo_legal_move.move_type == MoveType::Castle {
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

    // can only move king, no castling though
    } else {

        for pseudo_legal_move in moves {

            if pseudo_legal_move.moved_piece == own_king && attacked_squares & 1 << pseudo_legal_move.end_square == 0
            && pseudo_legal_move.move_type != MoveType::Castle {
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
    if !is_back_two_ranks(colour.opposite(), king_square as usize) {

        let forward = colour.offset_index(king_square as usize);

        let test_square1 = forward - 1;
        let test_square2 = forward + 1;

        if board.get_piece_abs(test_square1) as u8 == opp_colour | PAWN {
            attackers.push(1 << test_square1);

        } else if board.get_piece_abs(test_square2) as u8 == opp_colour | PAWN {
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
    let opp_colour = colour.opposite();

    let pawn = colour as u8 | PAWN;
    let backward = opp_colour.offset_index(square);

    if board.get_piece_abs(backward - 1) as u8 == pawn || 
        board.get_piece_abs(backward + 1) as u8 == pawn {
        return true;
    }

    let knight = KNIGHT | colour as u8;

    for offset in KNIGHT_OFFSETS {
        if board.get_piece_abs((sq + offset) as usize) as u8 == knight {
            return true;
        }
    }

    let bishop = BISHOP | colour as u8;
    let queen = QUEEN | colour as u8;

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
    
    let rook = ROOK | colour as u8;

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
    
    let king = KING | colour as u8;

    for offset in KING_OFFSETS {
        if board.get_piece_abs((sq + offset) as usize) as u8 == king {
            return true;
        }
    }

    return false;

}

// bitboard :)
pub fn get_attacked_squares_surrounding_king(board: &Board, colour: Colour) -> u128 {

    let mut attacked_squares = 0;

    let king = match colour {
        White => board.white_king,
        Black => board.black_king
    } as isize;

    for offset in KING_OFFSETS {

        let test_square = (king + offset) as usize;

        if test_square > H1 {
            break;
        }

        if board.get_piece_abs(test_square) == Border || is_attacking_square(test_square, board, colour.opposite()) {
            attacked_squares |= 1 << test_square;
        }
    }

    match colour {
        White => {
            if board.castling_rights & WHITE_KINGSIDE != NO_CASTLING_RIGHTS && is_attacking_square(C1, board, Black) {
                attacked_squares |= 1 << C1;
            }
            if board.castling_rights & WHITE_QUEENSIDE != NO_CASTLING_RIGHTS && is_attacking_square(G1, board, Black) {
                attacked_squares |= 1 << G1;
            }
        },
        Black => {
            if board.castling_rights & BLACK_KINGSIDE != NO_CASTLING_RIGHTS && is_attacking_square(C8, board, White) {
                attacked_squares |= 1 << C8;
            }
            if board.castling_rights & BLACK_QUEENSIDE != NO_CASTLING_RIGHTS && is_attacking_square(G8, board, White) {
                attacked_squares |= 1 << G8;
            }
        }
    }

    return attacked_squares;

}

pub fn get_num_moves(board: &mut Board, depth: u32) -> u32 {

    if depth == 0 {
        return 1;
    }

    let possible_moves = get_possible_moves(board);
    
    if depth == 1 {
        return possible_moves.len() as u32;
    }

    let mut moves = 0;

    for possible_move in &possible_moves {
        board.make_move(possible_move);
        moves += get_num_moves(board, depth - 1);
        board.undo_move();
    }

    return moves;

}