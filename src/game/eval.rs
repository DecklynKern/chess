#![allow(non_upper_case_globals)]
use super::chess_util::*;
use super::piece::*;
use super::board::*;
use super::r#move::*;

pub fn is_back_rank(colour: Colour, square: Square) -> bool {
    (colour == White && square >= A1) || (colour == Black && square <= H8)
}

pub fn is_back_two_ranks(colour: Colour, square: Square) -> bool {
    (colour == White && square >= A2) || (colour == Black && square <= H7)
}

fn try_add_move(moves: &mut Vec<Move>, board: &Board, start_square: Square, offset: i8) -> bool {

    let end_square = start_square.wrapping_add_signed(offset);

    if !square_is_on_board(end_square) {
        return false;
    }
    
    let end_piece = board.get_piece(end_square);

    let end_piece_empty = end_piece == Empty;
    let same_colour = board.get_piece(start_square).same_colour(end_piece);

    if !end_piece_empty && same_colour {
        return false;
    }

    let new_move = Move::new(board, start_square, end_square);
    let is_empty = new_move.replaced_piece == Empty;
    moves.push(new_move);

    is_empty

}

fn add_pawn_moves(moves: &mut Vec<Move>, board:&Board, start_square: Square, end_square: Square, colour: u8, is_promo: bool) {
    if is_promo {
        moves.push(Move::new_promotion(board, start_square, end_square, (colour | QUEEN).into()));
        moves.push(Move::new_promotion(board, start_square, end_square, (colour | ROOK).into()));
        moves.push(Move::new_promotion(board, start_square, end_square, (colour | BISHOP).into()));
        moves.push(Move::new_promotion(board, start_square, end_square, (colour | KNIGHT).into()));
    }
    else {
        moves.push(Move::new(board, start_square, end_square));
    }
}

fn gen_valid_pawn_moves(moves: &mut Vec<Move>, board:&Board, start_square: Square, colour: Colour) {

    let forward_square = colour.offset_rank(start_square);

    let file = start_square % 16;
    let is_promo = is_back_rank(colour.opposite(), forward_square);

    if file != 0 {

        let test_square = forward_square - 1;
        let piece_at = board.get_piece(test_square);

        if piece_at != Piece::Empty && piece_at.get_colour() != colour {
            add_pawn_moves(moves, board, start_square, test_square, colour as u8, is_promo);
        }
    }

    if file != 7 {

        let test_square = forward_square + 1;
        let piece_at = board.get_piece(test_square);

        if piece_at != Piece::Empty && piece_at.get_colour() != colour {
            add_pawn_moves(moves, board, start_square, test_square, colour as u8, is_promo);
        }
    }

    if board.get_piece(forward_square) == Piece::Empty {

        add_pawn_moves(moves, board, start_square, forward_square, colour as u8, is_promo);

        let double_move_square = colour.offset_rank(forward_square);

        if board.get_piece(double_move_square) == Empty && is_back_two_ranks(colour, start_square) {
            moves.push(Move::new_pawn_double(board, start_square, double_move_square));
        }
    }

    if let Some(en_passant_square) = board.en_passant_chance {
        if (en_passant_square as i8 - colour.offset_rank(start_square) as i8).abs() == 1 {
            moves.push(Move::new_en_passant(board, start_square, en_passant_square))
        }
    }

}

fn add_bishop_moves(moves: &mut Vec<Move>, board: &Board, start_square: Square) {
    
    for offset in DIAGONAL_OFFSETS {

        let mut total_offset = offset;

        while try_add_move(moves, board, start_square, total_offset) {
            total_offset += offset;
        }
    }
}

fn add_rook_moves(moves: &mut Vec<Move>, board: &Board, start_square: Square) {
    
    for offset in ORTHOGONAL_OFFSETS {

        let mut total_offset = offset;

        while try_add_move(moves, board, start_square, total_offset) {
            total_offset += offset;
        }
    }
}

pub fn get_possible_moves(board: &Board) -> Vec<Move> {

    let side_to_move = board.side_to_move;
    let mut moves: Vec<Move> = Vec::with_capacity(32);

    let king_square: Square;
    let pawn: Piece;
    let knight: Piece;
    let bishop: Piece;
    let rook: Piece;
    let queen: Piece;
    let own_king: Piece;

    if side_to_move == White {

        pawn = WhitePawn;
        knight = WhiteKnight;
        bishop = WhiteBishop;
        rook = WhiteRook;
        queen = WhiteQueen;

        king_square = board.white_king;
        own_king = WhiteKing;

        if board.castling_rights & WHITE_KINGSIDE != NO_CASTLING_RIGHTS && board.get_piece(F1) == Empty && board.get_piece(G1) == Empty &&
        !is_attacking_square(F1, board, Black) && !is_attacking_square(G1, board, Black) {
            moves.push(Move::new_castle(board, king_square, G1));
        }
        if board.castling_rights & WHITE_QUEENSIDE != NO_CASTLING_RIGHTS && board.get_piece(B1) == Empty && board.get_piece(C1) == Empty &&
        board.get_piece(D1) == Empty && !is_attacking_square(C1, board, Black) && !is_attacking_square(D1, board, Black) {
            moves.push(Move::new_castle(board, king_square, C1));
        }
    }
    else {

        pawn = BlackPawn;
        knight = BlackKnight;
        bishop = BlackBishop;
        rook = BlackRook;
        queen = BlackQueen;

        king_square = board.black_king;
        own_king = BlackKing;

        if board.castling_rights & BLACK_KINGSIDE != NO_CASTLING_RIGHTS && board.get_piece(F8) == Empty && board.get_piece(G8) == Empty &&
        !is_attacking_square(F8, board, White) && !is_attacking_square(G8, board, White) {
            moves.push(Move::new_castle(board, king_square, G8));
        }
        if board.castling_rights & BLACK_QUEENSIDE != NO_CASTLING_RIGHTS && board.get_piece(B8) == Empty && board.get_piece(C8) == Empty &&
        board.get_piece(D8) == Empty && !is_attacking_square(C8, board, White) && !is_attacking_square(D8, board, White) {
            moves.push(Move::new_castle(board, king_square, C8));
        }
    }

    for &pawn in &board.piece_positions[pawn as usize] {
        gen_valid_pawn_moves(&mut moves, board, pawn, side_to_move);
    }

    for &knight in &board.piece_positions[knight as usize] {
        for offset in KNIGHT_OFFSETS {
            try_add_move(&mut moves, board, knight, offset);
        }
    }

    for &bishop in &board.piece_positions[bishop as usize] {
        add_bishop_moves(&mut moves, board, bishop);
    }

    for &rook in &board.piece_positions[rook as usize] {
        add_rook_moves(&mut moves, board, rook);
    }

    for &queen in &board.piece_positions[queen as usize] {
        add_bishop_moves(&mut moves, board, queen);
        add_rook_moves(&mut moves, board, queen);
    }
    
    let king_surrounding_attacked_squares = get_attacked_squares_surrounding_king(board, side_to_move);

    // just prevent walking king into an attack to start with
    for offset in KING_OFFSETS {
        let test_square = king_square.wrapping_add_signed(offset);
        if test_square < 0x80 && king_surrounding_attacked_squares & (1 << test_square) == 0 {
            try_add_move(&mut moves, board, king_square, offset);
        }
    }

    let mut legal_moves: Vec<Move> = Vec::with_capacity(moves.capacity());
    
    let position_info = get_position_info(board, side_to_move);

    match position_info.king_attacker_count {
        0 => {
            
            // can't move king into check or move pinned pieces
            for pseudo_legal_move in moves {
    
                let mut is_pinned = false;
    
                for (pinned_square, safe_squares) in &position_info.pinned_pieces {
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
        1 => {
            
            // can only move king out of the way or block, no castling though
            for pseudo_legal_move in moves {
    
                if pseudo_legal_move.move_type == MoveType::Castle {
                    continue;
                }
    
                if pseudo_legal_move.moved_piece == own_king && pseudo_legal_move.move_type != MoveType::Castle ||
                pseudo_legal_move.moved_piece != own_king && position_info.king_block_board & 1 << pseudo_legal_move.end_square != 0 {
                    
                    let mut is_pinned = false;
    
                    for (pinned_square, safe_squares) in &position_info.pinned_pieces {
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
        }
        _ => {

            // can only move king, no castling though
            for pseudo_legal_move in moves {

                if pseudo_legal_move.moved_piece == own_king
                && pseudo_legal_move.move_type != MoveType::Castle {
                    legal_moves.push(pseudo_legal_move);
                }
            }
        }
    }

    legal_moves

}

pub struct PositionInfo {
    pub pinned_pieces: Vec<(Square, u128)>,
    pub king_attacker_count: u32,
    pub king_block_board: u128,
    pub opponent_attacked_squares: u128
}

pub fn get_position_info(board: &Board, colour: Colour) -> PositionInfo {
    
    let mut pinned_pieces = Vec::new();
    let mut king_attacker_count = 0;
    let mut king_block_board = 0;
    let mut opponent_attacked_squares = 0;

    let king_square = match colour {
        White => board.white_king,
        Black => board.black_king
    };

    let opp_colour = colour.opposite() as u8;

    let opp_pawn = (PAWN | opp_colour).into();

    for pawn in board.get_piece_position(opp_pawn) {
        //opponent_attacked_squares |= 
    }
    
    if !is_back_two_ranks(colour.opposite(), king_square as Square) {

        let forward = colour.offset_rank(king_square as Square);

        let test_square1 = forward - 1;
        let test_square2 = forward + 1;
        
        let opp_pawn = (opp_colour | PAWN).into();

        if board.get_piece(test_square1)  == opp_pawn {
            king_attacker_count += 1;
            king_block_board |= 1 << test_square1;
        }
        else if board.get_piece(test_square2)  == opp_pawn {
            king_attacker_count += 1;
            king_block_board |= 1 << test_square2;
        }
    }
    
    let opp_knight = KNIGHT | opp_colour;

    for offset in KNIGHT_OFFSETS {
        let test_square = king_square.wrapping_add_signed(offset);
        if board.get_piece(test_square) as u8 == opp_knight {
            king_attacker_count += 1;
            king_block_board |= 1 << test_square;
            break;
        }
    }
    
    enum PinAttack {
        NoneFound,
        Pin(Square),
        DoubleBlocked
    }

    let opp_bishop = BISHOP | opp_colour;
    let opp_queen = QUEEN | opp_colour;

    for dir in DIAGONAL_OFFSETS {
        
        let mut test_square = king_square;
        let mut line_squares = 0u128;
        let mut pin = PinAttack::NoneFound;
        
        loop {
            
            test_square = test_square.wrapping_add_signed(dir);
            
            if !square_is_on_board(test_square) {
                break;
            }

            let piece = board.get_piece(test_square);

            if piece == Empty {
                line_squares |= 1 << test_square;
                continue;
            }

            if piece.is_colour(colour) {
                
                pin = match pin {
                    PinAttack::NoneFound => PinAttack::Pin(test_square),
                    PinAttack::DoubleBlocked | PinAttack::Pin(_) => PinAttack::DoubleBlocked
                };
                
                continue;
                
            }

            if piece as u8 == opp_bishop || piece as u8 == opp_queen {
                
                line_squares |= 1 << test_square;
                
                match pin {
                    PinAttack::NoneFound => {
                        king_block_board |= line_squares;
                        king_attacker_count += 1;
                    }
                    PinAttack::Pin(pin_square) => {
                        pinned_pieces.push((pin_square, line_squares))
                    }
                    PinAttack::DoubleBlocked => {}
                }
            }

            break;

        }
    }

    let opp_rook = ROOK | opp_colour;

    for dir in ORTHOGONAL_OFFSETS {
        
        let mut test_square = king_square;
        let mut line_squares = 0u128;
        let mut pin = PinAttack::NoneFound;
        
        loop {
            
            test_square = test_square.wrapping_add_signed(dir);
            
            if !square_is_on_board(test_square) {
                break;
            }

            let piece = board.get_piece(test_square);

            if piece == Empty {
                line_squares |= 1 << test_square;
                continue;
            }

            if piece.is_colour(colour) {
                
                pin = match pin {
                    PinAttack::NoneFound => PinAttack::Pin(test_square),
                    PinAttack::DoubleBlocked | PinAttack::Pin(_) => PinAttack::DoubleBlocked
                };
                
                continue;
                
            }

            if piece as u8 == opp_rook || piece as u8 == opp_queen {
                
                line_squares |= 1 << test_square;
                
                match pin {
                    PinAttack::NoneFound => {
                        king_block_board |= line_squares;
                        king_attacker_count += 1;
                    }
                    PinAttack::Pin(pin_square) => {
                        pinned_pieces.push((pin_square, line_squares))
                    }
                    PinAttack::DoubleBlocked => {}
                }
            }

            break;

        }
    }

    PositionInfo {
        pinned_pieces,
        king_attacker_count,
        king_block_board,
        opponent_attacked_squares
    }
}

pub fn is_attacking_square(square: Square, board: &Board, colour: Colour) -> bool {

    let opp_colour = colour.opposite();

    let pawn = colour as u8 | PAWN;
    let backward = opp_colour.offset_rank(square);

    if board.get_piece(backward - 1) as u8 == pawn || 
        board.get_piece(backward + 1) as u8 == pawn {
        return true;
    }

    let knight = KNIGHT | colour as u8;

    for offset in KNIGHT_OFFSETS {
        if board.get_piece(square.wrapping_add_signed(offset)) as u8 == knight {
            return true;
        }
    }

    let bishop = BISHOP | colour as u8;
    let queen = QUEEN | colour as u8;

    for dir in DIAGONAL_OFFSETS {
        
        let mut test_square = square;
        
        loop {
            
            test_square = test_square.wrapping_add_signed(dir);

            let piece = board.get_piece(test_square);

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
        
        let mut test_square = square;
        
        loop {
            
            test_square = test_square.wrapping_add_signed(dir);

            let piece = board.get_piece(test_square as Square);

            if piece == Empty || piece.is_king() {
                continue;
            }

            if piece as u8 == rook || piece as u8 == queen {
                return true;
            }

            break;
            
        }
    }
    
    let king = (KING | colour as u8).into();

    for offset in KING_OFFSETS {
        if board.get_piece(square.wrapping_add_signed(offset)) == king {
            return true;
        }
    }

    false

}

// bitboard :)
pub fn get_attacked_squares_surrounding_king(board: &Board, colour: Colour) -> u128 {

    let mut attacked_squares = 0;

    let king = match colour {
        White => board.white_king,
        Black => board.black_king
    };

    for offset in KING_OFFSETS {

        let test_square = king.wrapping_add_signed(offset);

        if !square_is_on_board(test_square) {
            continue;
        }
        
        if is_attacking_square(test_square, board, colour.opposite()) {
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
        }
        Black => {
            if board.castling_rights & BLACK_KINGSIDE != NO_CASTLING_RIGHTS && is_attacking_square(C8, board, White) {
                attacked_squares |= 1 << C8;
            }
            if board.castling_rights & BLACK_QUEENSIDE != NO_CASTLING_RIGHTS && is_attacking_square(G8, board, White) {
                attacked_squares |= 1 << G8;
            }
        }
    }

    attacked_squares

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
