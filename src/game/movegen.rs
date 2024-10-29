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

fn gen_valid_pawn_moves(moves: &mut Vec<Move>, board: &Board, start_square: Square, colour: Colour) {

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

        if is_back_two_ranks(colour, start_square) && board.get_piece(double_move_square) == Empty {
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
            total_offset = total_offset.wrapping_add(offset);
        }
    }
}

fn add_rook_moves(moves: &mut Vec<Move>, board: &Board, start_square: Square) {
    
    for offset in ORTHOGONAL_OFFSETS {

        let mut total_offset = offset;

        while try_add_move(moves, board, start_square, total_offset) {
            total_offset = total_offset.wrapping_add(offset);
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
    
    let position_info = get_position_info(board, side_to_move);

    if side_to_move == White {

        pawn = WhitePawn;
        knight = WhiteKnight;
        bishop = WhiteBishop;
        rook = WhiteRook;
        queen = WhiteQueen;

        king_square = board.white_king;
        own_king = WhiteKing;

        if board.castling_rights & WHITE_KINGSIDE != NO_CASTLING_RIGHTS && board.get_piece(F1) == Empty && board.get_piece(G1) == Empty &&
        position_info.opponent_attacked_squares & (1 << F1 | 1 << G1) == 0 {
            moves.push(Move::new_castle(board, king_square, G1));
        }
        if board.castling_rights & WHITE_QUEENSIDE != NO_CASTLING_RIGHTS && board.get_piece(B1) == Empty && board.get_piece(C1) == Empty &&
        board.get_piece(D1) == Empty && position_info.opponent_attacked_squares & (1 << C1 | 1 << D1) == 0 {
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
        position_info.opponent_attacked_squares & (1 << F8 | 1 << G8) == 0 {
            moves.push(Move::new_castle(board, king_square, G8));
        }
        if board.castling_rights & BLACK_QUEENSIDE != NO_CASTLING_RIGHTS && board.get_piece(B8) == Empty && board.get_piece(C8) == Empty &&
        board.get_piece(D8) == Empty && position_info.opponent_attacked_squares & (1 << C8 | 1 << D8) == 0 {
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

    // just prevent walking king into an attack to start with
    for offset in KING_OFFSETS {
        let test_square = king_square.wrapping_add_signed(offset);
        if test_square < 0x80 && position_info.opponent_attacked_squares & (1 << test_square) == 0 {
            try_add_move(&mut moves, board, king_square, offset);
        }
    }

    let mut legal_moves: Vec<Move> = Vec::with_capacity(moves.capacity());

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
        1 if position_info.king_block_board != 0 => {
            
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

    let king_square = match colour {
        White => board.white_king,
        Black => board.black_king
    };

    let mut opponent_attacked_squares = get_king_move_board(match colour {
        White => board.black_king,
        Black => board.white_king
    });

    let king_square_board = 1 << king_square;

    let opp_colour = colour.opposite() as u8;

    let opp_pawn = (PAWN | opp_colour).into();

    for pawn_square in board.get_piece_position(opp_pawn) {

        let attack_board = get_pawn_attack_board(*pawn_square, colour.opposite());
        opponent_attacked_squares |= attack_board;

        if attack_board & king_square_board != 0 {
            king_block_board = 1 << pawn_square;
        }
    }

    if king_square_board & opponent_attacked_squares != 0 {
        king_attacker_count = 1;
    }

    let opp_knight = KNIGHT | opp_colour;
    let mut knight_attack_board = 0;

    for knight_square in board.get_piece_position(opp_knight.into()) {

        let attack_board = get_knight_move_board(*knight_square);
        knight_attack_board |= attack_board;

        if attack_board & king_square_board != 0 {
            king_block_board |= 1 << knight_square;
        }
    }

    if king_square_board & knight_attack_board != 0 {
        king_attacker_count += 1;
    }

    opponent_attacked_squares |= knight_attack_board;

    let opp_bishop = BISHOP | opp_colour;
    let opp_queen = QUEEN | opp_colour;
    let opp_rook = ROOK | opp_colour;

    let opp_bishop_positions = board.get_piece_position(opp_bishop.into());
    let opp_rook_positions = board.get_piece_position(opp_rook.into());
    let opp_queen_positions = board.get_piece_position(opp_queen.into());

    let opp_diagonal_positions = opp_bishop_positions
        .iter()
        .chain(opp_queen_positions)
        .cloned();
    
    let opp_orthogonal_positions = opp_rook_positions
        .iter()
        .chain(opp_queen_positions)
        .cloned();

    opponent_attacked_squares |= calc_sliding_boards(
        board,
        opp_diagonal_positions,
        &DIAGONAL_OFFSETS,
        king_square,
        &mut king_block_board,
        &mut king_attacker_count,
        &mut pinned_pieces
    );

    opponent_attacked_squares |= calc_sliding_boards(
        board,
        opp_orthogonal_positions,
        &ORTHOGONAL_OFFSETS,
        king_square,
        &mut king_block_board,
        &mut king_attacker_count,
        &mut pinned_pieces
    );

    PositionInfo {
        pinned_pieces,
        king_attacker_count,
        king_block_board,
        opponent_attacked_squares
    }
}

#[derive(PartialEq, Eq)]
enum PinAttack {
    NoneFound,
    Pin(Square),
    DoubleBlocked,
    ThroughKing
}

#[inline(always)]
fn calc_sliding_boards(
    board: &Board, piece_squares: impl Iterator<Item = Square>, offsets: &[i8; 4], king_square: Square, king_block_board: &mut u128,
    king_attacker_count: &mut u32, pinned_pieces: &mut Vec<(Square, u128)>
) -> u128 {

    let mut attack_board = 0;

    for piece_square in piece_squares {

        for &dir in offsets {
        
            let mut test_square = piece_square;
            let mut line_squares = 0;
            let mut pin = PinAttack::NoneFound;
        
            loop {
                
                test_square = test_square.wrapping_add_signed(dir);
                
                if !square_is_on_board(test_square) {
                    break;
                }

                line_squares |= 1 << test_square;
    
                if test_square == king_square {
                    
                    match pin {
                        PinAttack::NoneFound => {
                            *king_block_board |= line_squares | 1 << piece_square;
                            *king_attacker_count += 1;
                            pin = PinAttack::ThroughKing;
                        }
                        PinAttack::Pin(pin_square) => {
                            pinned_pieces.push((pin_square, line_squares | 1 << piece_square))
                        }
                        _ => {}
                    }

                    continue;

                }
    
                let piece = board.get_piece(test_square);
    
                if piece == Empty {
                }
                else if piece.is_colour(board.side_to_move) {
                    
                    pin = match pin {
                        PinAttack::NoneFound => {
                            attack_board |= line_squares;
                            PinAttack::Pin(test_square)
                        }
                        PinAttack::Pin(_) => PinAttack::DoubleBlocked,
                        _ => pin
                    };
                }
                else {
                    break;
                }
            }

            if pin == PinAttack::NoneFound || pin == PinAttack::ThroughKing {
                attack_board |= line_squares;
            }
        }
    }

    attack_board

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