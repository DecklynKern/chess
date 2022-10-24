#![allow(non_upper_case_globals)]
use super::piece::*;
use super::r#move::*;
use super::chess_util::*;

use array_init;

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

    pub fn get_piece_counts(&self, colour: Colour) -> (u64, u64, u64, u64, u64, u64) {
        (
            self.piece_positions[(colour as u8 | PAWN) as usize].len() as u64,
            self.piece_positions[(colour as u8 | KNIGHT) as usize].len() as u64,
            self.piece_positions[(colour as u8 | BISHOP) as usize].len() as u64,
            self.piece_positions[(colour as u8 | ROOK) as usize].len() as u64,
            self.piece_positions[(colour as u8 | QUEEN) as usize].len() as u64,
            1
        )
    }

    pub fn from_fen(f: String) -> Board {

        let mut chars = f.chars(); 
        let mut setup_board = [Border; 144];
        
        let mut pos = A8;

        let mut white_king = 0;
        let mut black_king = 0;

        let mut piece_positions = array_init::array_init(|_| Vec::new());

        while pos <= H1 {

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

        let move_colour = self.side_to_move;
        let opp_colour = move_colour.opposite();

        if move_to_make.replaced_piece == Empty {
            self.board.swap(move_to_make.start_square, move_to_make.end_square);

        } else {

            self.board[move_to_make.end_square] = move_to_make.moved_piece;
            self.board[move_to_make.start_square] = Empty;

            del_vec(&mut self.piece_positions[move_to_make.replaced_piece as usize], move_to_make.end_square);

        }

        self.en_passant_chance = None;

        replace_vec(&mut self.piece_positions[move_to_make.moved_piece as usize], move_to_make.start_square, move_to_make.end_square);

        match move_to_make.move_type {
            MoveType::PawnDouble => {
                self.en_passant_chance = Some(opp_colour.offset_index(move_to_make.end_square));
            },
            MoveType::EnPassant => {
                let captured_square = opp_colour.offset_index(move_to_make.end_square);
                self.board[captured_square] = Empty;
                del_vec(&mut self.piece_positions[(opp_colour as u8 | PAWN) as usize], captured_square);
            },
            MoveType::Promotion(promote_to) => {
                self.board[move_to_make.end_square] = promote_to;
                del_vec(&mut self.piece_positions[(move_colour as u8 | PAWN) as usize], move_to_make.end_square);
                self.piece_positions[promote_to as usize].push(move_to_make.end_square);
            },
            MoveType::Castle => {
                let (rook_start_square, rook_end_square) = if move_to_make.end_square % 12 < 6 {
                    (move_to_make.end_square - 2, move_to_make.end_square + 1)
                } else {
                    (move_to_make.end_square + 1, move_to_make.end_square - 1)
                };
                replace_vec(&mut self.piece_positions[(move_colour as u8 | ROOK) as usize], rook_start_square, rook_end_square);
                self.board.swap(rook_start_square, rook_end_square);
            },
            MoveType::Normal => {}
        };

        match move_to_make.moved_piece {
            WhiteRook if move_to_make.start_square == A1 => self.castling_rights.1 = false,
            WhiteRook if move_to_make.start_square == H1 => self.castling_rights.0 = false,
            BlackRook if move_to_make.start_square == A8 => self.castling_rights.3 = false,
            BlackRook if move_to_make.start_square == H8 => self.castling_rights.2 = false,
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

        match move_to_make.end_square {
            A1 => self.castling_rights.1 = false,
            H1 => self.castling_rights.0 = false,
            A8 => self.castling_rights.3 = false,
            H8 => self.castling_rights.2 = false,
            _ => {}
        }

        self.side_to_move = opp_colour;
        self.previous_moves.push(*move_to_make);
        self.turns_taken += 1;

    }

    pub fn undo_move(&mut self) {

        let opp_colour = self.side_to_move;
        let move_colour = opp_colour.opposite();

        let move_to_undo = match self.previous_moves.pop() {
            Some(last_move) => last_move,
            None => return
        };

        if move_to_undo.replaced_piece == Empty {
            self.board.swap(move_to_undo.start_square, move_to_undo.end_square);

        } else {
            
            self.board[move_to_undo.start_square] = move_to_undo.moved_piece;
            self.board[move_to_undo.end_square] = move_to_undo.replaced_piece;

            self.piece_positions[move_to_undo.replaced_piece as usize].push(move_to_undo.end_square);

        }

        match move_to_undo.moved_piece {
            WhiteKing => self.white_king = move_to_undo.start_square,
            BlackKing => self.black_king = move_to_undo.start_square,
            _ => {}
        }

        self.en_passant_chance = if self.previous_moves.is_empty() {
            None
        } else {
            let prev_move = self.previous_moves.last().unwrap();
            match prev_move.move_type {
                MoveType::PawnDouble =>Some(move_colour.offset_index(prev_move.end_square)),
                _ => None
            }
        };

        match move_to_undo.move_type {
            MoveType::PawnDouble => {},
            MoveType::EnPassant => {
                let captured_square = opp_colour.offset_index(move_to_undo.end_square);
                self.board[captured_square] = Piece::from_num(opp_colour as u8 | PAWN);
                self.piece_positions[(opp_colour as u8 | PAWN) as usize].push(captured_square);
            },
            MoveType::Promotion(promote_to) => {
                self.board[move_to_undo.start_square] = Piece::from_num(move_colour as u8 | PAWN);
                self.piece_positions[(move_colour as u8 | PAWN) as usize].push(move_to_undo.end_square);
                del_vec(&mut self.piece_positions[promote_to as usize], move_to_undo.end_square);
            },
            MoveType::Castle => {
                let (rook_start_square, rook_end_square) = if move_to_undo.end_square % 12 < 6 {
                    (move_to_undo.end_square - 2, move_to_undo.end_square + 1)
                } else {
                    (move_to_undo.end_square + 1, move_to_undo.end_square - 1)
                };
                replace_vec(&mut self.piece_positions[(move_colour as u8 | ROOK) as usize], rook_end_square, rook_start_square);
                self.board.swap(rook_start_square, rook_end_square);
            }
            MoveType::Normal => {}
        }

        replace_vec(&mut self.piece_positions[move_to_undo.moved_piece as usize], move_to_undo.end_square, move_to_undo.start_square);

        self.castling_rights = move_to_undo.old_castling_rights;
        self.side_to_move = move_colour;
        self.turns_taken -= 1;

    }
}