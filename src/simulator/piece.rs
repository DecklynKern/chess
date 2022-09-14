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
            _ => unreachable!()
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
    King {colour: Colour}
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
            Piece::Empty => unreachable!()
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
                _ => unreachable!()
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