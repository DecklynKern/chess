pub type CastlingRights = u8;

pub const ALL_CASTLING_RIGHTS: CastlingRights = 0b1111;
pub const NO_CASTLING_RIGHTS: CastlingRights = 0b0000;

pub const WHITE_KINGSIDE: CastlingRights = 0b1000;
pub const WHITE_QUEENSIDE: CastlingRights = 0b0100;
pub const BLACK_KINGSIDE: CastlingRights = 0b0010;
pub const BLACK_QUEENSIDE: CastlingRights = 0b0001;

pub type Square = u8;

pub const A8: Square = 0x00;
pub const B8: Square = 0x01;
pub const C8: Square = 0x02;
pub const D8: Square = 0x03;
pub const E8: Square = 0x04;
pub const F8: Square = 0x05;
pub const G8: Square = 0x06;
pub const H8: Square = 0x07;

pub const A7: Square = 0x10;
pub const B7: Square = 0x11;
pub const C7: Square = 0x12;
pub const D7: Square = 0x13;
pub const E7: Square = 0x14;
pub const F7: Square = 0x15;
pub const G7: Square = 0x16;
pub const H7: Square = 0x17;

pub const A6: Square = 0x20;
pub const B6: Square = 0x21;
pub const C6: Square = 0x22;
pub const D6: Square = 0x23;
pub const E6: Square = 0x24;
pub const F6: Square = 0x25;
pub const G6: Square = 0x26;
pub const H6: Square = 0x27;

pub const A5: Square = 0x30;
pub const B5: Square = 0x31;
pub const C5: Square = 0x32;
pub const D5: Square = 0x33;
pub const E5: Square = 0x34;
pub const F5: Square = 0x35;
pub const G5: Square = 0x36;
pub const H5: Square = 0x37;

pub const A4: Square = 0x40;
pub const B4: Square = 0x41;
pub const C4: Square = 0x42;
pub const D4: Square = 0x43;
pub const E4: Square = 0x44;
pub const F4: Square = 0x45;
pub const G4: Square = 0x46;
pub const H4: Square = 0x47;

pub const A3: Square = 0x50;
pub const B3: Square = 0x51;
pub const C3: Square = 0x52;
pub const D3: Square = 0x53;
pub const E3: Square = 0x54;
pub const F3: Square = 0x55;
pub const G3: Square = 0x56;
pub const H3: Square = 0x57;

pub const A2: Square = 0x60;
pub const B2: Square = 0x61;
pub const C2: Square = 0x62;
pub const D2: Square = 0x63;
pub const E2: Square = 0x64;
pub const F2: Square = 0x65;
pub const G2: Square = 0x66;
pub const H2: Square = 0x67;

pub const A1: Square = 0x70;
pub const B1: Square = 0x71;
pub const C1: Square = 0x72;
pub const D1: Square = 0x73;
pub const E1: Square = 0x74;
pub const F1: Square = 0x75;
pub const G1: Square = 0x76;
pub const H1: Square = 0x77;

pub const VALID_SQUARES: [Square; 64] = [
    A8, B8, C8, D8, E8, F8, G8, H8, 
    A7, B7, C7, D7, E7, F7, G7, H7, 
    A6, B6, C6, D6, E6, F6, G6, H6, 
    A5, B5, C5, D5, E5, F5, G5, H5, 
    A4, B4, C4, D4, E4, F4, G4, H4,
    A3, B3, C3, D3, E3, F3, G3, H3, 
    A2, B2, C2, D2, E2, F2, G2, H2, 
    A1, B1, C1, D1, E1, F1, G1, H1, 
];

pub const KNIGHT_OFFSETS: [i8; 8] = [-33, -31, -18, -14, 14, 18, 31, 33];
pub const KING_OFFSETS: [i8; 8] = [-17, -16, -15, -1, 1, 15, 16, 17];
pub const ORTHOGONAL_OFFSETS: [i8; 4] = [-16, -1, 1, 16];
pub const DIAGONAL_OFFSETS: [i8; 4] = [-17, -15, 15, 17];

pub static mut WHITE_PAWN_MOVE_BOARDS: [u128; 128] = [0; 128];
pub static mut BLACK_PAWN_MOVE_BOARDS: [u128; 128] = [0; 128];
pub static mut KNIGHT_MOVE_BOARDS: [u128; 128] = [0; 128];
pub static mut KING_MOVE_BOARDS: [u128; 128] = [0; 128];

pub fn load_move_boards() {
    
    for square in VALID_SQUARES {
        
        let mut white_pawn_move_board = 0;
        let mut black_pawn_move_board = 0;
        let mut knight_move_board = 0;
        let mut king_move_board = 0;
        
        for offset in KNIGHT_OFFSETS {
            let test_square = square.wrapping_add_signed(offset);
            if square_is_on_board(test_square) {
                knight_move_board |= 1 << test_square;
            }
        }

        for offset in KING_OFFSETS {
            let test_square = square.wrapping_add_signed(offset);
            if square_is_on_board(test_square) {
                king_move_board |= 1 << test_square;
            }
        }
        
        unsafe {
            WHITE_PAWN_MOVE_BOARDS[square as usize] = white_pawn_move_board;
            BLACK_PAWN_MOVE_BOARDS[square as usize] = black_pawn_move_board;
            KNIGHT_MOVE_BOARDS[square as usize] = knight_move_board;
            KING_MOVE_BOARDS[square as usize] = king_move_board;
        }
    }
}

pub fn get_knight_move_board(square: Square) -> u128 {
    unsafe {
        KNIGHT_MOVE_BOARDS[square as usize]
    }
}

pub fn get_king_move_board(square: Square) -> u128 {
    unsafe {
        KING_MOVE_BOARDS[square as usize]
    }
}

pub fn square_is_on_board(square: Square) -> bool {
    (square & 0x88) == 0
}

pub fn an_to_square(an: String) -> Square {
    
    let mut chars = an.chars();
    (
        chars.next().unwrap().to_digit(18).unwrap() - 10 
        + 16 * (8 - chars.next().unwrap().to_digit(10).unwrap())
    ) as Square
    
}

pub fn square_to_an(square: Square) -> String {
    
    format!(
        "{}{}",
        [
            'a',
            'b',
            'c',
            'd',
            'e',
            'f',
            'g',
            'h'
        ][square as usize % 8],
        8 - square / 16
    )
}