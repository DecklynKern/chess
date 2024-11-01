#![allow(non_upper_case_globals)]

use super::Square;

pub const White: Colour = Colour::White;
pub const Black: Colour = Colour::Black;

pub const WhitePawn: Piece = Piece::WhitePawn;
pub const WhiteKnight: Piece = Piece::WhiteKnight;
pub const WhiteBishop: Piece = Piece::WhiteBishop;
pub const WhiteRook: Piece = Piece::WhiteRook;
pub const WhiteQueen: Piece = Piece::WhiteQueen;
pub const WhiteKing: Piece = Piece::WhiteKing;

pub const BlackPawn: Piece = Piece::BlackPawn;
pub const BlackKnight: Piece = Piece::BlackKnight;
pub const BlackBishop: Piece = Piece::BlackBishop;
pub const BlackRook: Piece = Piece::BlackRook;
pub const BlackQueen: Piece = Piece::BlackQueen;
pub const BlackKing: Piece = Piece::BlackKing;

pub const Empty: Piece = Piece::Empty;

pub const WHITE: u8 = 0b0000;
pub const BLACK: u8 = 0b1000;

pub const PAWN: u8 = 0b000;
pub const KNIGHT: u8 = 0b001;
pub const BISHOP: u8 = 0b010;
pub const ROOK: u8 = 0b011;
pub const QUEEN: u8 = 0b100;
pub const KING: u8 = 0b101;

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub enum Colour {
    White = 0b0000,
    Black = 0b1000
}

impl Colour {

    pub fn from_char(c: char) -> Self {
        match c {
            'w' => White,
            'b' => Black,
            _ => unreachable!()
        }
    }

    pub fn to_char(self) -> char {
        match self {
            White => 'w',
            Black => 'b'
        }
    }

    pub fn opposite(self) -> Self {
        if self == Black {
            White
        }
        else {
            Black
        }
    }

    pub fn offset_rank(self, square: Square) -> Square {
        match self {
            White => square.overflowing_sub(16).0,
            Black => square.overflowing_add(16).0
        }
    }

    pub fn to_dir(self) -> isize {
        match self {
            White => -1,
            Black => 1
        }
    }

}

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub enum Piece {
    WhitePawn = 0b0000,
    WhiteKnight = 0b0001,
    WhiteBishop = 0b0010,
    WhiteRook = 0b0011,
    WhiteQueen = 0b0100,
    WhiteKing = 0b0101,
    BlackPawn = 0b1000,
    BlackKnight = 0b1001,
    BlackBishop = 0b1010,
    BlackRook = 0b1011,
    BlackQueen = 0b1100,
    BlackKing = 0b1101,
    Empty = 0b1111
}

impl Piece {

    pub fn get_colour(self) -> Colour {
        match (self as u8) & 0b1000 {
            WHITE => White,
            BLACK => Black,
            _ => unreachable!()
        }
    }

    pub fn is_colour(self, colour: Colour) -> bool {
        self as u8 & BLACK == colour as u8
    }

    pub fn same_colour(self, other: Piece) -> bool {
        self as u8 >> 3 == other as u8 >> 3
    }

    pub fn from_char(c: char) -> Self {
        match c {
            'P' => WhitePawn,
            'N' => WhiteKnight,
            'B' => WhiteBishop,
            'R' => WhiteRook,
            'Q' => WhiteQueen,
            'K' => WhiteKing,
            'p' => BlackPawn,
            'n' => BlackKnight,
            'b' => BlackBishop,
            'r' => BlackRook,
            'q' => BlackQueen,
            'k' => BlackKing,
            ' ' => Empty,
            _ => unreachable!()
        }
    }

    pub fn to_char(self) -> char {
        return match self {
            WhitePawn => 'P',
            WhiteKnight => 'N',
            WhiteBishop => 'B',
            WhiteRook => 'R',
            WhiteQueen => 'Q',
            WhiteKing => 'K',
            BlackPawn => 'p',
            BlackKnight => 'n',
            BlackBishop => 'b',
            BlackRook => 'r',
            BlackQueen => 'q',
            BlackKing => 'k',
            Empty => ' '
        }
    }

    pub fn to_an_char(self) -> char {
        return match self {
            WhitePawn | BlackPawn => 'P',
            WhiteKnight | BlackKnight => 'N',
            WhiteBishop | BlackBishop => 'B',
            WhiteRook | BlackRook => 'R',
            WhiteQueen | BlackQueen => 'Q',
            WhiteKing | BlackKing => 'K',
            Empty => ' '
        }
    }

    pub fn is_pawn(self) -> bool {
        (self as u8) & 0b111 == PAWN
    }

    pub fn is_knight(self) -> bool {
        (self as u8) & 0b111 == KNIGHT
    }

    pub fn is_bishop(self) -> bool {
        (self as u8) & 0b111 == BISHOP
    }

    pub fn is_rook(self) -> bool {
        (self as u8) & 0b111 == ROOK
    }

    pub fn is_queen(self) -> bool {
        (self as u8) & 0b111 == QUEEN
    }

    pub fn is_king(self) -> bool {
        (self as u8) & 0b111 == KING
    }
}

impl From<u8> for Piece {
    fn from(value: u8) -> Self {
        match value {
            0b0000 => WhitePawn,
            0b0001 => WhiteKnight,
            0b0010 => WhiteBishop,
            0b0011 => WhiteRook,
            0b0100 => WhiteQueen,
            0b0101 => WhiteKing,
            0b1000 => BlackPawn,
            0b1001 => BlackKnight,
            0b1010 => BlackBishop,
            0b1011 => BlackRook,
            0b1100 => BlackQueen,
            0b1101 => BlackKing,
            0b1110 => Empty,
            _ => unreachable!()
        }
    }
}