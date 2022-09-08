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
}

impl Board {

    pub fn from_fen (f: String) -> Board {

        let mut chars = f.chars(); 

        const INIT: Piece = Piece::Empty; // thanks https://github.com/rust-lang/rust/issues/44796
        let mut setup_board: [Piece; 64] = [INIT; 64];
        
        let mut pos = 0;
        let mut chars_parsed = 0usize;

        for c in chars.clone() {

            chars_parsed += 1;

            if c.is_alphabetic() {
                setup_board[pos] = Piece::from_char(c);
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

        Board {
            board: setup_board,
            side_to_move: Colour::from_char(chars.nth(chars_parsed + 1).unwrap()),
            turns_taken: 0, //not correct
            previous_moves: Vec::new()
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

        return fen;

    }

    pub fn make_move(&mut self, move_to_make: &Move) {

        if move_to_make.replaced_piece != Piece::Empty {
            self.board[move_to_make.end_square] = Piece::Empty;
        }

        self.board.swap(move_to_make.start_square, move_to_make.end_square);

        self.side_to_move = self.side_to_move.opposite();
        self.previous_moves.push(*move_to_make);
        self.turns_taken += 1;

    }

    pub fn undo_move(&mut self) {

        let move_to_undo = self.previous_moves.pop().unwrap();

        if move_to_undo.replaced_piece != Piece::Empty {
            self.board[move_to_undo.start_square] = move_to_undo.replaced_piece;
        }

        self.board.swap(move_to_undo.start_square, move_to_undo.end_square);

        self.side_to_move = self.side_to_move.opposite();
        self.turns_taken -= 1;

    }

}