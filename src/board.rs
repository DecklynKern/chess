use crate::eval::*;

#[derive(Copy, Clone, PartialEq)]
pub enum Colour {
    Black, 
    White
}

impl Colour {

    pub fn from_char(c: char) -> Colour {
        match c {
            'w' => Colour::White,
            'b' => Colour::Black,
            _ => Colour::Black // should not happen
        }
    }

    pub fn to_char(&self) -> char {
        match self {
            Colour::White => 'w',
            Colour::Black => 'b'
        }
    }

    pub fn opposite(self) -> Colour {
        if self == Colour::Black {Colour::White} else {Colour::Black}
    }

}

#[derive(Copy, Clone, PartialEq)]
pub enum Piece {
    Empty,
    Pawn {colour: Colour},
    Knight {colour: Colour},
    Bishop {colour: Colour},
    Rook {colour: Colour},
    Queen {colour: Colour},
    King {colour: Colour},
}

impl Piece {

    pub fn get_colour(&self) -> Colour {
        match *self {
            Piece::Pawn{colour} => colour,
            Piece::Knight{colour} => colour,
            Piece::Bishop{colour} => colour,
            Piece::Rook{colour} => colour,
            Piece::Queen{colour} => colour,
            Piece::King{colour} => colour,
            Piece::Empty => Colour::Black // should not happen
        }
    }

    pub fn from_char(c: char) -> Piece {

        if c == ' ' {
            return Piece::Empty;
        }

        let colour = match c.is_uppercase() {
            true => Colour::White,
            false => Colour::Black,
        };

        return match c.to_lowercase().last().unwrap() {
                'p' => Piece::Pawn{colour},
                'n' => Piece::Knight{colour},
                'b' => Piece::Bishop{colour},
                'r' => Piece::Rook{colour},
                'q' => Piece::Queen{colour},
                'k' => Piece::King{colour},
                _ => Piece::Pawn{colour} // should not happen
        };
    }

    pub fn to_char(&self) -> char {

        return match self {
            Piece::Empty => ' ',
            _ => {

                let mut c = match self {
                    Piece::Pawn{colour: _} => 'p',
                    Piece::Knight{colour: _} => 'n',
                    Piece::Bishop{colour: _} => 'b',
                    Piece::Rook{colour: _} => 'r',
                    Piece::King{colour: _} => 'k',
                    Piece::Queen{colour: _} => 'q',
                    Piece::Empty => ' '
                };

                if self.get_colour() == Colour::White {
                    c = c.to_uppercase().last().unwrap();
                }
                
                c
            }
        }

    }
}

pub struct Board {
    pub board: [Piece; 64],
    pub side_to_move: Colour,
    pub turns_taken: usize,
    pub previous_moves: Vec<Move>,
    pub en_passant_chance: Option<usize>,
    pub castling_rights: (bool, bool, bool, bool),
    pub white_king: usize,
    pub black_king: usize,
}

impl Board {

    pub fn from_fen (f: String) -> Board {

        let mut chars = f.chars(); 

        const INIT: Piece = Piece::Empty; // thanks https://github.com/rust-lang/rust/issues/44796
        let mut setup_board: [Piece; 64] = [INIT; 64];
        
        let mut pos = 0;

        let mut white_king = 0;
        let mut black_king = 0;

        while pos < 64 {

            let c = chars.next().unwrap();

            if c.is_alphabetic() {

                let piece = Piece::from_char(c);

                setup_board[pos] = piece;

                match piece {
                    Piece::King{colour: Colour::White} => {white_king = pos},
                    Piece::King{colour: Colour::Black} => {black_king = pos},
                    _ => {}
                }

                pos += 1;

            } else if c.is_numeric() {
                pos += c.to_digit(10).unwrap() as usize;
            
            } else {
                assert_eq!(c, '/');
            }

            if pos == 64 {
                break;
            }

        }

        chars.next();
        let side_to_move = Colour::from_char(chars.next().unwrap());
        chars.next();

        let (mut white_queenside_castle, mut white_kingside_castle, mut black_queenside_castle, mut black_kingside_castle) = (false, false, false, false); 

        let mut c = 'a';

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

        Board {
            board: setup_board,
            side_to_move: side_to_move,
            turns_taken: 0, //not correct
            previous_moves: Vec::new(),
            en_passant_chance: None,
            castling_rights: (white_kingside_castle, white_queenside_castle, black_kingside_castle, black_queenside_castle),
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
                
                match self.board[col + row * 8] {

                    Piece::Empty => {spaces += 1},

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

        return fen;

    }

    pub fn make_move(&mut self, move_to_make: &Move) {

        if move_to_make.replaced_piece != Piece::Empty {
            self.board[move_to_make.end_square] = Piece::Empty;
        }

        match move_to_make.moved_piece {
            Piece::King{colour: Colour::White} => {self.white_king = move_to_make.end_square},
            Piece::King{colour: Colour::Black} => {self.black_king = move_to_make.end_square},
            _ => {}
        }

        self.board.swap(move_to_make.start_square, move_to_make.end_square);

        if move_to_make.special_move_type == SpecialMoveType::EnPassant {
            match move_to_make.moved_piece {
                Piece::Pawn{colour: Colour::White} => {
                    self.board[move_to_make.end_square + 8] = Piece::Empty;
                },
                Piece::Pawn{colour: Colour::Black} => {
                    self.board[move_to_make.end_square - 8] = Piece::Empty;
                },
                _ => {}
            }
        }

        self.side_to_move = self.side_to_move.opposite();
        self.previous_moves.push(*move_to_make);
        self.turns_taken += 1;

        self.en_passant_chance = match move_to_make.moved_piece {
            Piece::Pawn{colour: _} if (move_to_make.start_square as isize - move_to_make.end_square as isize).abs() == 16 => {Some(move_to_make.end_square)},
            _ => None
        }

    }

    pub fn undo_move(&mut self) {

        let move_to_undo = self.previous_moves.pop().unwrap();

        if move_to_undo.replaced_piece != Piece::Empty {
            self.board[move_to_undo.start_square] = move_to_undo.replaced_piece;
        }

        match move_to_undo.moved_piece {
            Piece::King{colour: Colour::White} => {self.white_king = move_to_undo.start_square},
            Piece::King{colour: Colour::Black} => {self.black_king = move_to_undo.start_square},
            _ => {}
        }

        if move_to_undo.special_move_type == SpecialMoveType::EnPassant {
            match move_to_undo.moved_piece {
                Piece::Pawn{colour: Colour::White} => {
                    self.board[move_to_undo.end_square + 8] = Piece::Pawn{colour: Colour::Black};
                },
                Piece::Pawn{colour: Colour::Black} => {
                    self.board[move_to_undo.end_square - 8] = Piece::Pawn{colour: Colour::White};
                },
                _ => {}
            }
        }

        self.board.swap(move_to_undo.start_square, move_to_undo.end_square);

        self.side_to_move = self.side_to_move.opposite();
        self.turns_taken -= 1;
        self.en_passant_chance = move_to_undo.old_en_passant_chance;

    }

}