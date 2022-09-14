use crate::simulator::piece::*;
use crate::simulator::eval::*;

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

    pub fn default() -> Board {
        Board::from_fen(String::from("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1"))
    }

    pub fn from_fen(f: String) -> Board {

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

    pub fn index_to_an(idx: usize) -> String {

        let rank = 8 - idx / 8;
        let file = String::from("abcdefgh").chars().nth(idx % 8).unwrap();
    
        return format!("{}{}", file, rank);

    }

    pub fn long_an_to_index(long_an: String) -> usize {

        let mut chars = long_an.chars();
        0usize + match chars.next().unwrap() {
            'a' => 0,
            'b' => 1,
            'c' => 2,
            'd' => 3,
            'e' => 4,
            'f' => 5,
            'g' => 6,
            'h' => 7,
            _ => unreachable!()
        } + 8 * (chars.next().unwrap().to_digit(10).unwrap() as usize - 1)
    }

    pub fn index_to_long_an(idx: usize) -> String {
        format!("{}{}", match idx % 8 {
            0 => "a",
            1 => "b",
            2 => "c",
            3 => "d",
            4 => "e",
            5 => "f",
            6 => "g",
            7 => "h",
            _ => unreachable!()
        }, (idx / 8).to_string())

    }

    pub fn get_piece_counts(&self) -> (usize, usize, usize, usize, usize, usize) {

        let mut piece_counts = (0, 0, 0, 0, 0, 0); 

        for square in 0..64 {
            match self.board[square] {
                Piece::Pawn{colour: _} => piece_counts.0 += 1,
                Piece::Knight{colour: _} => piece_counts.1 += 1,
                Piece::Bishop{colour: _} => piece_counts.2 += 1,
                Piece::Rook{colour: _} => piece_counts.3 += 1,
                Piece::Queen{colour: _} => piece_counts.4 += 1,
                Piece::King{colour: _} => piece_counts.5 += 1,
                _ => {}
            }
        }

        return piece_counts;

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

        match (move_to_make.is_en_passant, move_to_make.is_castle, move_to_make.moved_piece.get_colour()) {
            (false, false, _) => {},
            (true, false, Colour::White) => {
                self.board[move_to_make.end_square + 8] = Piece::Empty;
            },
            (true, false, Colour::Black) => {
                self.board[move_to_make.end_square - 8] = Piece::Empty;
            },
            (false, true, Colour::White) => {
                if move_to_make.end_square == 58 {
                    self.board[56] = Piece::Empty;
                    self.board[59] = Piece::Rook{colour: Colour::White};
                } else {
                    self.board[63] = Piece::Empty;
                    self.board[61] = Piece::Rook{colour: Colour::White};
                }
                self.castling_rights.0 = false;
                self.castling_rights.1 = false;
            },
            (false, true, Colour::Black) => {
                if move_to_make.end_square == 2 {
                    self.board[0] = Piece::Empty;
                    self.board[3] = Piece::Rook{colour: Colour::Black};
                } else {
                    self.board[7] = Piece::Empty;
                    self.board[5] = Piece::Rook{colour: Colour::Black};
                }
                self.castling_rights.2 = false;
                self.castling_rights.3 = false;
            },
            _ => unreachable!()
        };

        match move_to_make.moved_piece {
            Piece::Pawn{colour} => {
                
                if (move_to_make.start_square as isize - move_to_make.end_square as isize).abs() == 16 {
                    self.en_passant_chance = Some(move_to_make.end_square);
                
                } else if colour == Colour::White && move_to_make.end_square < 8 {
                    self.board[move_to_make.end_square] = Piece::Queen{colour: Colour::White};

                } else if colour == Colour::Black && move_to_make.end_square > 55 {
                    self.board[move_to_make.end_square] = Piece::Queen{colour: Colour::Black};
                }   

            },
            Piece::King{colour: Colour::White} => {
                self.castling_rights.0 = false;
                self.castling_rights.1 = false;
            },
            Piece::King{colour: Colour::Black} => {
                self.castling_rights.2 = false;
                self.castling_rights.3 = false;
            },
            Piece::Rook{colour: Colour::White} if move_to_make.start_square == 56 => {
                self.castling_rights.1 = false;
            }
            Piece::Rook{colour: Colour::White} if move_to_make.start_square == 63 => {
                self.castling_rights.0 = false;
            }
            Piece::Rook{colour: Colour::Black} if move_to_make.start_square == 0 => {
                self.castling_rights.3 = false;
            }
            Piece::Rook{colour: Colour::Black} if move_to_make.start_square == 7 => {
                self.castling_rights.2 = false;
            }
            _ => {}
        }

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

        match move_to_undo.moved_piece {
            Piece::Pawn{colour} => {
                if colour == Colour::White && move_to_undo.end_square < 8 {
                    self.board[move_to_undo.start_square] = Piece::Pawn{colour: Colour::White};

                } else if colour == Colour::Black && move_to_undo.end_square > 55 {
                    self.board[move_to_undo.start_square] = Piece::Pawn{colour: Colour::Black};
                }   

            },
            Piece::King{colour: Colour::White} => {self.white_king = move_to_undo.start_square},
            Piece::King{colour: Colour::Black} => {self.black_king = move_to_undo.start_square},
            _ => {}
        }

        match (move_to_undo.is_en_passant, move_to_undo.is_castle, move_to_undo.moved_piece.get_colour()) {
            (false, false, _) => {}
            (true, false, Colour::White) => {
                self.board[move_to_undo.end_square + 8] = Piece::Pawn{colour: Colour::Black};
            },
            (true, false, Colour::Black) => {
                self.board[move_to_undo.end_square - 8] = Piece::Pawn{colour: Colour::White};
            },
            (false, true, Colour::White) => {
                if move_to_undo.end_square == 58 {
                    self.board[56] = Piece::Rook{colour: Colour::White};
                    self.board[59] = Piece::Empty;
                } else {
                    self.board[63] = Piece::Rook{colour: Colour::White};
                    self.board[61] = Piece::Empty;
                }
            }
            (false, true, Colour::Black) => {
                if move_to_undo.end_square == 2 {
                    self.board[0] = Piece::Rook{colour: Colour::Black};
                    self.board[3] = Piece::Empty;
                } else {
                    self.board[7] = Piece::Rook{colour: Colour::Black};
                    self.board[5] = Piece::Empty;
                }
            },
            _ => unreachable!()
        }

        self.side_to_move = self.side_to_move.opposite();
        self.turns_taken -= 1;
        self.en_passant_chance = move_to_undo.old_en_passant_chance;
        self.castling_rights = move_to_undo.old_castling_rights;

    }
}