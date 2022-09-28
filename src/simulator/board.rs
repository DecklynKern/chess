use crate::simulator::piece::*;
use crate::simulator::eval::*;

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

pub struct Board {
    board: [Piece; 144],
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
        let mut setup_board = [BORDER; 144];
        
        let mut pos = 26;

        let mut white_king = 0;
        let mut black_king = 0;

        while pos < 118 {

            let c = chars.next().unwrap();

            if c.is_alphabetic() {

                let piece = Piece::from_char(c);

                setup_board[pos] = piece;

                match piece {
                    WHITE_KING => {white_king = pos},
                    BLACK_KING => {black_king = pos},
                    _ => {}
                }

                pos += 1;

            } else if c.is_numeric() {
                for _ in 0..c.to_digit(10).unwrap() as usize {
                    setup_board[pos] = EMPTY;
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
                
                match self.get_piece(row, col) {

                    EMPTY => {spaces += 1},

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
            Some(square) => (8 - Board::pos_to_row_col(square).0).to_string(),
            None => String::from("-")
        }.as_str();

        fen += " 0 ";

        return fen + self.turns_taken.to_string().as_str(); // implement 50 turn rule someday

    }

    pub fn make_move(&mut self, move_to_make: &Move) {

        if move_to_make.replaced_piece == EMPTY {
            self.board.swap(move_to_make.start_square, move_to_make.end_square);

        } else {
            self.board[move_to_make.end_square] = move_to_make.moved_piece;
            self.board[move_to_make.start_square] = EMPTY;
        }

        match (move_to_make.move_type, move_to_make.moved_piece.get_colour()) {
            (SpecialMoveType::Normal, _) => {},
            (SpecialMoveType::EnPassant, Colour::White) => {
                self.board[move_to_make.end_square + 12] = EMPTY;
            },
            (SpecialMoveType::EnPassant, Colour::Black) => {
                self.board[move_to_make.end_square - 12] = EMPTY;
            },
            (SpecialMoveType::Castle, _) => {
                if move_to_make.end_square % 12 < 6 { // slightly compressed to remove branch, might cause bugs
                    self.board.swap(move_to_make.end_square - 2, move_to_make.end_square + 1);
                } else {
                    self.board.swap(move_to_make.end_square - 1, move_to_make.end_square + 1);
                }
            },
        };

        self.en_passant_chance = None;

        match move_to_make.moved_piece {
            Piece::Pawn{colour} => {
                
                if (move_to_make.start_square as isize - move_to_make.end_square as isize).abs() == 24 {
                    self.en_passant_chance = Some(move_to_make.end_square);
                
                } else if colour == Colour::White && move_to_make.end_square < 36 {
                    self.board[move_to_make.end_square] = WHITE_QUEEN;

                } else if colour == Colour::Black && move_to_make.end_square > 108 {
                    self.board[move_to_make.end_square] = BLACK_QUEEN;
                }   

            },
            WHITE_KING => {
                self.castling_rights.0 = false;
                self.castling_rights.1 = false;
                self.white_king = move_to_make.end_square;
            },
            BLACK_KING => {
                self.castling_rights.2 = false;
                self.castling_rights.3 = false;
                self.black_king = move_to_make.end_square;
            },
            WHITE_ROOK if move_to_make.start_square == 110 => {
                self.castling_rights.1 = false;
            }
            WHITE_ROOK if move_to_make.start_square == 117 => {
                self.castling_rights.0 = false;
            }
            BLACK_ROOK if move_to_make.start_square == 26 => {
                self.castling_rights.3 = false;
            }
            BLACK_ROOK if move_to_make.start_square == 33 => {
                self.castling_rights.2 = false;
            }
            _ => {}
        }

        self.side_to_move = self.side_to_move.opposite();
        self.previous_moves.push(*move_to_make);
        self.turns_taken += 1;

    }

    pub fn undo_move(&mut self) {

        let move_to_undo: Move;

        match self.previous_moves.pop() {
            Some(last_move) => {
                move_to_undo = last_move;
            },
            None => return
        }

        if move_to_undo.replaced_piece == EMPTY {
            self.board.swap(move_to_undo.start_square, move_to_undo.end_square);

        } else {
            self.board[move_to_undo.start_square] = move_to_undo.moved_piece;
            self.board[move_to_undo.end_square] = move_to_undo.replaced_piece;
        }

        match move_to_undo.moved_piece {
            Piece::Pawn{colour} => {
                if colour == Colour::White && move_to_undo.end_square < 36 {
                    self.board[move_to_undo.start_square] = WHITE_PAWN;

                } else if colour == Colour::Black && move_to_undo.end_square > 108 {
                    self.board[move_to_undo.start_square] = BLACK_PAWN;
                }   

            },
            WHITE_KING => {self.white_king = move_to_undo.start_square},
            BLACK_KING => {self.black_king = move_to_undo.start_square},
            _ => {}
        }

        match (move_to_undo.move_type, move_to_undo.moved_piece.get_colour()) {
            (SpecialMoveType::Normal, _) => {}
            (SpecialMoveType::EnPassant, Colour::White) => {
                self.board[move_to_undo.end_square + 12] = BLACK_PAWN;
            },
            (SpecialMoveType::EnPassant, Colour::Black) => {
                self.board[move_to_undo.end_square - 12] = WHITE_PAWN;
            },
            (SpecialMoveType::Castle, _) => {
                if move_to_undo.end_square % 12 < 6 { // slightly compressed to remove branch, might cause bugs
                    self.board.swap(move_to_undo.end_square - 2, move_to_undo.end_square + 1);
                } else {
                    self.board.swap(move_to_undo.end_square - 1, move_to_undo.end_square + 1);
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