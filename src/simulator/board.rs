#![allow(non_upper_case_globals)]
use crate::simulator::piece::*;
use crate::simulator::eval::*;
use array_init;

use super::chess_util;
use super::chess_util::index_to_long_an;
use super::chess_util::long_an_to_index;

pub const VALID_SQUARES: [usize; 64] = [
     26,  27,  28,  29,  30,  31,  32,  33, 
     38,  39,  40,  41,  42,  43,  44,  45, 
     50,  51,  52,  53,  54,  55,  56,  57, 
     62,  63,  64,  65,  66,  67,  68,  69, 
     74,  75,  76,  77,  78,  79,  80,  81, 
     86,  87,  88,  89,  90,  91,  92,  93, 
     98,  99, 100, 101, 102, 103, 104, 105, 
    110, 111, 112, 113, 114, 115, 116, 117
];

fn replace_vec<T>(vec: &mut Vec<T>, val: T, new_val: T)
where T: PartialEq {
    let idx = vec.iter().position(|x| *x == val).unwrap();
    vec[idx] = new_val;
}

fn del_vec<T>(vec: &mut Vec<T>, val: T)
where T: PartialEq {
    let idx = vec.iter().position(|x| *x == val).unwrap();
    vec.remove(idx);
}

pub struct Board {
    board: [Piece; 144],
    pub side_to_move: Colour,
    pub turns_taken: usize,
    pub previous_moves: Vec<Move>,
    pub en_passant_chance: Option<usize>,
    pub castling_rights: (bool, bool, bool, bool),
    pub piece_positions: [Vec<usize>; 16],
    pub white_king: usize,
    pub black_king: usize,
}

impl Board {

    pub fn default() -> Board {
        Board::from_fen(String::from("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1"))
    }

    pub fn get_piece(&self, row: usize, col: usize) -> Piece {
        self.board[row * 12 + col + 26]
    }

    pub fn get_piece_abs(&self, pos: usize) -> Piece {
        self.board[pos]
    }

    pub fn pos_to_row_col(pos: usize) -> (usize, usize) {
        (pos / 12 - 2, pos % 12 - 2)
    }

    pub fn from_fen(f: String) -> Board {

        let mut chars = f.chars(); 
        let mut setup_board = [Border; 144];
        
        let mut pos = 26;

        let mut white_king = 0;
        let mut black_king = 0;

        let mut piece_positions = array_init::array_init(|_| Vec::new());

        while pos < 118 {

            let c = chars.next().unwrap();

            if c.is_alphabetic() {

                let piece = Piece::from_char(c);

                setup_board[pos] = piece;

                match piece {
                    WhiteKing => white_king = pos,
                    BlackKing => black_king = pos,
                    _ => {}
                };

                piece_positions[piece as usize].push(pos);

                pos += 1;

            } else if c.is_numeric() {
                for _ in 0..c.to_digit(10).unwrap() as usize {
                    setup_board[pos] = Empty;
                    pos += 1;
                }
            
            } else {
                assert_eq!(c, '/');
                pos += 4;
            }
        }

        chars.next();
        let side_to_move = Colour::from_char(chars.next().unwrap());
        chars.next();

        let (mut white_queenside_castle, mut white_kingside_castle, mut black_queenside_castle, mut black_kingside_castle) = (false, false, false, false); 

        let mut c;

        loop {
            c = chars.next().unwrap();
            match c {
                'K' => white_kingside_castle = true,
                'Q' => white_queenside_castle = true,
                'k' => black_kingside_castle = true,
                'q' => black_queenside_castle = true,
                ' ' => break,
                _ => {}
            }
        }

        let c = chars.next().unwrap();

        let en_passant_chance = if c != '-' {
            let mut index = String::new();
            index.push(c);
            index.push(chars.next().unwrap());
            Some(long_an_to_index(index))
        } else {
            None
        };

        Board {
            board: setup_board,
            side_to_move: side_to_move,
            turns_taken: 0, //not correct
            previous_moves: Vec::new(),
            en_passant_chance: en_passant_chance,
            castling_rights: (white_kingside_castle, white_queenside_castle, black_kingside_castle, black_queenside_castle),
            piece_positions: piece_positions,
            white_king: white_king,
            black_king: black_king
        }
    }

    pub fn to_fen(&self) -> String {
        
        let mut fen = "".to_owned();
        
        for row in 0..8 {

            if row != 0 {
                fen += "/";
            }

            let mut spaces = 0;

            for col in 0..8 {
                
                match self.get_piece(row, col) {

                    Empty => {spaces += 1},

                    piece => {

                        if spaces > 0 {
                            fen += &spaces.to_string();
                            spaces = 0;
                        }

                        fen += &piece.to_char().to_string();

                    }

                }
            }

            if spaces > 0 {
                fen += &spaces.to_string();
            }

        }

        fen += " ";
        fen +=  &self.side_to_move.to_char().to_string();

        fen += " ";

        if self.castling_rights == (false, false, false, false) {
            fen += "-";
        }

        if self.castling_rights.0 {
            fen += "K";
        }

        if self.castling_rights.1 {
            fen += "Q";
        }

        if self.castling_rights.2 {
            fen += "k";
        }

        if self.castling_rights.3 {
            fen += "q";
        }

        fen += " ";

        fen += match self.en_passant_chance {
            Some(square) => index_to_long_an(square),
            None => String::from("-")
        }.as_str();

        fen += " 0 ";

        return fen + self.turns_taken.to_string().as_str(); // implement 50 turn rule someday

    }

    pub fn make_move(&mut self, move_to_make: &Move) {

        if move_to_make.replaced_piece == Empty {
            self.board.swap(move_to_make.start_square, move_to_make.end_square);

        } else {

            self.board[move_to_make.end_square] = move_to_make.moved_piece;
            self.board[move_to_make.start_square] = Empty;

            del_vec(&mut self.piece_positions[move_to_make.replaced_piece as usize], move_to_make.end_square);

        }

        replace_vec(&mut self.piece_positions[move_to_make.moved_piece as usize], move_to_make.start_square, move_to_make.end_square);

        match (move_to_make.move_type, move_to_make.moved_piece.get_colour()) {
            (SpecialMoveType::Normal, _) => {},
            (SpecialMoveType::EnPassant, White) => {
                let captured_square = move_to_make.end_square + 12;
                self.board[captured_square] = Empty;
                del_vec(&mut self.piece_positions[(BLACK | PAWN) as usize], captured_square);
            },
            (SpecialMoveType::EnPassant, Black) => {
                let captured_square = move_to_make.end_square - 12;
                self.board[captured_square] = Empty;
                del_vec(&mut self.piece_positions[(WHITE | PAWN) as usize], captured_square);
            },
            (SpecialMoveType::Castle, _) => {
                let (rook_start_square, rook_end_square) = if move_to_make.end_square % 12 < 6 {
                    (move_to_make.end_square - 2, move_to_make.end_square + 1)
                } else {
                    (move_to_make.end_square + 1, move_to_make.end_square - 1)
                };
                replace_vec(&mut self.piece_positions[(self.side_to_move as u8 | ROOK) as usize], rook_start_square, rook_end_square);
                self.board.swap(rook_start_square, rook_end_square);
            },
        };

        self.en_passant_chance = None;

        match move_to_make.moved_piece {
            WhitePawn if move_to_make.start_square == move_to_make.end_square + 24 => self.en_passant_chance = Some(move_to_make.end_square + 12),
            WhitePawn if move_to_make.end_square < 36 => self.board[move_to_make.end_square] = WhiteQueen,
            BlackPawn if move_to_make.start_square == move_to_make.end_square - 24 => self.en_passant_chance = Some(move_to_make.end_square - 12),
            BlackPawn if move_to_make.end_square > 108 => self.board[move_to_make.end_square] = BlackQueen,
            WhiteRook if move_to_make.start_square == 110 => self.castling_rights.1 = false,
            WhiteRook if move_to_make.start_square == 117 => self.castling_rights.0 = false,
            BlackRook if move_to_make.start_square == 26 => self.castling_rights.3 = false,
            BlackRook if move_to_make.start_square == 33 => self.castling_rights.2 = false,
            WhiteKing => {
                self.castling_rights.0 = false;
                self.castling_rights.1 = false;
                self.white_king = move_to_make.end_square;
            },
            BlackKing => {
                self.castling_rights.2 = false;
                self.castling_rights.3 = false;
                self.black_king = move_to_make.end_square;
            },
            _ => {}
        }

        match (move_to_make.replaced_piece, move_to_make.end_square) {
            (WhiteRook, 110) => self.castling_rights.1 = false,
            (WhiteRook, 117) => self.castling_rights.0 = false,
            (BlackRook, 26) => self.castling_rights.3 = false,
            (BlackRook, 33) => self.castling_rights.2 = false,
            _ => {}
        }

        self.side_to_move = self.side_to_move.opposite();
        self.previous_moves.push(*move_to_make);
        self.turns_taken += 1;

    }

    pub fn undo_move(&mut self) {

        let move_to_undo: Move;

        match self.previous_moves.pop() {
            Some(last_move) => move_to_undo = last_move,
            None => return
        }

        if move_to_undo.replaced_piece == Empty {
            self.board.swap(move_to_undo.start_square, move_to_undo.end_square);

        } else {
            
            self.board[move_to_undo.start_square] = move_to_undo.moved_piece;
            self.board[move_to_undo.end_square] = move_to_undo.replaced_piece;

            self.piece_positions[move_to_undo.replaced_piece as usize].push(move_to_undo.end_square);

        }

        replace_vec(&mut self.piece_positions[move_to_undo.moved_piece as usize], move_to_undo.end_square, move_to_undo.start_square);

        match move_to_undo.moved_piece {
            WhitePawn if move_to_undo.end_square < 36 => self.board[move_to_undo.start_square] = WhitePawn,
            BlackPawn if move_to_undo.end_square > 108 => self.board[move_to_undo.start_square] = BlackPawn,
            WhiteKing => self.white_king = move_to_undo.start_square,
            BlackKing => self.black_king = move_to_undo.start_square,
            _ => {}
        }

        match (move_to_undo.move_type, move_to_undo.moved_piece.get_colour()) {
            (SpecialMoveType::Normal, _) => {},
            (SpecialMoveType::EnPassant, White) => {
                let captured_square = move_to_undo.end_square + 12;
                self.board[captured_square] = BlackPawn;
                self.piece_positions[(BLACK | PAWN) as usize].push(captured_square);
            },
            (SpecialMoveType::EnPassant, Black) => {
                let captured_square = move_to_undo.end_square - 12;
                self.board[captured_square] = WhitePawn;
                self.piece_positions[(WHITE | PAWN) as usize].push(captured_square);
            },
            (SpecialMoveType::Castle, _) => {
                let (rook_start_square, rook_end_square) = if move_to_undo.end_square % 12 < 6 {
                    (move_to_undo.end_square - 2, move_to_undo.end_square + 1)
                } else {
                    (move_to_undo.end_square + 1, move_to_undo.end_square - 1)
                };
                replace_vec(&mut self.piece_positions[(self.side_to_move.opposite() as u8 | ROOK) as usize], rook_end_square, rook_start_square);
                self.board.swap(rook_start_square, rook_end_square);
            }
        }

        self.side_to_move = self.side_to_move.opposite();
        self.turns_taken -= 1;
        self.en_passant_chance = move_to_undo.old_en_passant_chance;
        self.castling_rights = move_to_undo.old_castling_rights;

    }
}