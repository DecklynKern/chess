#![allow(non_upper_case_globals)]
use super::piece::*;
use super::r#move::*;
use super::chess_util::*;

fn replace_vec<T>(vec: &mut [T], val: T, new_val: T)
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
    board: [Piece; 128],
    pub side_to_move: Colour,
    pub turns_taken: u32,
    pub previous_moves: Vec<Move>,
    pub en_passant_chance: Option<Square>,
    pub castling_rights: CastlingRights,
    pub piece_positions: [Vec<Square>; 16],
    pub white_king: Square,
    pub black_king: Square
}

impl Board {

    pub fn default() -> Self {
        Self::from_fen(String::from("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1"))
    }

    pub fn from_fen(f: String) -> Self {

        let mut chars = f.chars(); 
        let mut setup_board = [Empty; 128];
        
        let mut pos = A8 as usize;

        let mut white_king = 0;
        let mut black_king = 0;

        let mut piece_positions = array_init::array_init(|_| Vec::new());

        while pos <= H1 as usize {

            let c = chars.next().unwrap();

            if c.is_alphabetic() {

                let piece = Piece::from_char(c);

                setup_board[pos] = piece;

                match piece {
                    WhiteKing => white_king = pos as Square,
                    BlackKing => black_king = pos as Square,
                    _ => {}
                };

                piece_positions[piece as usize].push(pos as Square);

                pos += 1;
                
            }
            else if c.is_numeric() {
                for _ in 0..c.to_digit(10).unwrap() as usize {
                    setup_board[pos] = Empty;
                    pos += 1;
                }
            }
            else {
                assert_eq!(c, '/');
                pos += 8;
            }
        }

        chars.next();
        let side_to_move = Colour::from_char(chars.next().unwrap());
        chars.next();

        let mut castling_rights = NO_CASTLING_RIGHTS;

        let mut c;

        loop {
            c = chars.next().unwrap();
            match c {
                'K' => castling_rights |= WHITE_KINGSIDE,
                'Q' => castling_rights |= WHITE_QUEENSIDE,
                'k' => castling_rights |= BLACK_KINGSIDE,
                'q' => castling_rights |= BLACK_QUEENSIDE,
                ' ' => break,
                _ => {}
            }
        }

        let c = chars.next().unwrap();
        
        let en_passant_chance = (c != '-').then(|| {
            let mut index = String::new();
            index.push(c);
            index.push(chars.next().unwrap());
            an_to_square(index)
        });

        chars.next();

        let mut fifty_turn_count = "".to_owned();

        for char in chars.by_ref() {
            if char == ' ' {
                break;
            }
            fifty_turn_count += char.to_string().as_str();
        }

        let mut fullturn_num = "".to_owned();

        for char in chars.by_ref() {
            if char == ' ' {
                break;
            }
            fullturn_num += char.to_string().as_str();
        }

        Self {
            board: setup_board,
            side_to_move,
            turns_taken: match fullturn_num.parse::<u32>() {
                Ok(fullturns) => fullturns * 2 - 2 + if side_to_move == White {0} else {1},
                Err(_) => 0
            },
            previous_moves: Vec::new(),
            en_passant_chance,
            castling_rights,
            piece_positions,
            white_king,
            black_king
        }
    }

    pub fn get_fen(&self) -> String {
        
        let mut fen = "".to_owned();
        
        for row in 0..8 {

            if row != 0 {
                fen += "/";
            }

            let mut spaces = 0;

            for col in 0..8 {
                
                match self.get_piece_rc(row, col) {

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

        if self.castling_rights == NO_CASTLING_RIGHTS {
            fen += "-";
        }

        if self.castling_rights & WHITE_KINGSIDE != NO_CASTLING_RIGHTS {
            fen += "K";
        }

        if self.castling_rights & WHITE_QUEENSIDE != NO_CASTLING_RIGHTS {
            fen += "Q";
        }

        if self.castling_rights & BLACK_KINGSIDE != NO_CASTLING_RIGHTS {
            fen += "k";
        }

        if self.castling_rights & BLACK_QUEENSIDE != NO_CASTLING_RIGHTS {
            fen += "q";
        }

        fen += " ";

        fen += match self.en_passant_chance {
            Some(square) => square_to_an(square),
            None => String::from("-")
        }.as_str();

        fen += " 0 ";

        fen + (self.turns_taken / 2 + 1).to_string().as_str() // implement 50 turn rule someday

    }

    pub fn get_piece_rc(&self, row: usize, col: usize) -> Piece {
        unsafe {
            *self.board.get_unchecked(row * 16 + col)
        }
    }

    pub fn get_piece(&self, square: Square) -> Piece {
        unsafe {
            *self.board.get_unchecked(square as usize)
        }
    }
    
    pub fn set_piece(&mut self, square: Square, piece: Piece) {
        self.board[square as usize] = piece;
    }

    pub fn get_piece_position(&self, piece: Piece) -> &[Square] {
        unsafe {
            self.piece_positions.get_unchecked(piece as usize)
        }
    }

    pub fn get_piece_position_mut(&mut self, piece: Piece) -> &mut Vec<Square> {
        unsafe {
            self.piece_positions.get_unchecked_mut(piece as usize)
        }
    }

    pub fn get_piece_counts(&self, colour: Colour) -> [u32; 6] {
        [
            self.get_piece_position((colour as u8 | PAWN).into()).len() as u32,
            self.get_piece_position((colour as u8 | KNIGHT).into()).len() as u32,
            self.get_piece_position((colour as u8 | BISHOP).into()).len() as u32,
            self.get_piece_position((colour as u8 | ROOK).into()).len() as u32,
            self.get_piece_position((colour as u8 | QUEEN).into()).len() as u32,
            1
        ]
    }
    
    pub fn is_draw_by_insufficient_material(&self) -> bool {
        self.piece_positions[WhitePawn as usize].len() == 0 && 
        self.piece_positions[WhiteRook as usize].len() == 0 && 
        self.piece_positions[WhiteQueen as usize].len() == 0 && 
        self.piece_positions[WhiteBishop as usize].len() + self.piece_positions[WhiteKnight as usize].len() <= 1 && 
        self.piece_positions[BlackPawn as usize].len() == 0 && 
        self.piece_positions[BlackRook as usize].len() == 0 && 
        self.piece_positions[BlackQueen as usize].len() == 0 && 
        self.piece_positions[BlackBishop as usize].len() + self.piece_positions[WhiteKnight as usize].len() <= 1
    }

    pub fn make_move(&mut self, move_to_make: &Move) {

        let move_colour = self.side_to_move;
        let opp_colour = move_colour.opposite();
        
        match move_to_make.move_type {
            MoveType::EnPassant => {
                
                self.set_piece(move_to_make.start_square, Empty);
                self.set_piece(move_to_make.end_square, move_to_make.moved_piece);
            
                replace_vec(self.get_piece_position_mut(move_to_make.moved_piece), move_to_make.start_square, move_to_make.end_square);
                
                let captured_square = opp_colour.offset_rank(move_to_make.end_square);
                let captured_piece = self.get_piece(captured_square);
                self.set_piece(captured_square, Empty);
                del_vec(self.get_piece_position_mut(captured_piece), captured_square);
                
            }
            MoveType::Promotion(promote_to) => {
                
                self.set_piece(move_to_make.start_square, Empty);
                self.set_piece(move_to_make.end_square, promote_to);
                
                del_vec(self.get_piece_position_mut((move_colour as u8 | PAWN).into()), move_to_make.start_square);
                self.get_piece_position_mut(promote_to).push(move_to_make.end_square);
                
                if move_to_make.replaced_piece != Empty {
                    del_vec(self.get_piece_position_mut(move_to_make.replaced_piece), move_to_make.end_square);
                }
            }
            MoveType::Castle => {
                
                let rook = (move_colour as u8 | ROOK).into();
                let (rook_start_square, rook_end_square) = if move_to_make.end_square % 16 < 4 {
                    (move_to_make.end_square - 2, move_to_make.end_square + 1)
                }
                else {
                    (move_to_make.end_square + 1, move_to_make.end_square - 1)
                };
                
                self.set_piece(move_to_make.start_square, Empty);
                self.set_piece(move_to_make.end_square, move_to_make.moved_piece);
                self.set_piece(rook_start_square, Empty);
                self.set_piece(rook_end_square, rook);
                
                replace_vec(self.get_piece_position_mut(move_to_make.moved_piece), move_to_make.start_square, move_to_make.end_square);
                replace_vec(self.get_piece_position_mut(rook), rook_start_square, rook_end_square);
                
            }
            _ => {
                
                self.set_piece(move_to_make.start_square, Empty);
                self.set_piece(move_to_make.end_square, move_to_make.moved_piece);
            
                replace_vec(self.get_piece_position_mut(move_to_make.moved_piece), move_to_make.start_square, move_to_make.end_square);
                
                if move_to_make.replaced_piece != Empty {
                    del_vec(self.get_piece_position_mut(move_to_make.replaced_piece), move_to_make.end_square);
                }
            }
        }

        match move_to_make.moved_piece {
            WhiteRook if move_to_make.start_square == H1 => self.castling_rights &= !WHITE_KINGSIDE,
            WhiteRook if move_to_make.start_square == A1 => self.castling_rights &= !WHITE_QUEENSIDE,
            BlackRook if move_to_make.start_square == H8 => self.castling_rights &= !BLACK_KINGSIDE,
            BlackRook if move_to_make.start_square == A8 => self.castling_rights &= !BLACK_QUEENSIDE,
            WhiteKing => {
                self.castling_rights &= !(WHITE_KINGSIDE | WHITE_QUEENSIDE);
                self.white_king = move_to_make.end_square;
            }
            BlackKing => {
                self.castling_rights &= !(BLACK_KINGSIDE | BLACK_QUEENSIDE);
                self.black_king = move_to_make.end_square;
            }
            _ => {}
        }

        self.castling_rights &= match move_to_make.end_square {
            H1 => !WHITE_KINGSIDE,
            A1 => !WHITE_QUEENSIDE,
            H8 => !BLACK_KINGSIDE,
            A8 => !BLACK_QUEENSIDE,
            _ => ALL_CASTLING_RIGHTS
        };

        self.en_passant_chance = (move_to_make.move_type == MoveType::PawnDouble).then(|| opp_colour.offset_rank(move_to_make.end_square));
        self.side_to_move = opp_colour;
        self.previous_moves.push(move_to_make.clone());
        self.turns_taken += 1;

    }

    pub fn undo_move(&mut self) {

        let opp_colour = self.side_to_move;
        let move_colour = opp_colour.opposite();

        let Some(move_to_undo) = self.previous_moves.pop()
        else {
            return;
        };
        
        match move_to_undo.move_type {
            MoveType::EnPassant => {
                
                self.set_piece(move_to_undo.start_square, move_to_undo.moved_piece);
                self.set_piece(move_to_undo.end_square, Empty);
            
                replace_vec(self.get_piece_position_mut(move_to_undo.moved_piece), move_to_undo.end_square, move_to_undo.start_square);
                
                let captured_square = opp_colour.offset_rank(move_to_undo.end_square);
                let pawn = (opp_colour as u8 | PAWN).into();
                self.set_piece(captured_square, pawn);
                self.get_piece_position_mut(pawn).push(captured_square);
                
            }
            MoveType::Promotion(promote_to) => {
                
                let pawn = (move_colour as u8 | PAWN).into();
                
                self.set_piece(move_to_undo.start_square, pawn);
                self.set_piece(move_to_undo.end_square, move_to_undo.replaced_piece);
                
                self.get_piece_position_mut(pawn).push(move_to_undo.start_square);
                del_vec(self.get_piece_position_mut(promote_to), move_to_undo.end_square);
                
                if move_to_undo.replaced_piece != Empty {
                    self.get_piece_position_mut(move_to_undo.replaced_piece).push(move_to_undo.end_square);
                }
            }
            MoveType::Castle => {
                
                let rook = (move_colour as u8 | ROOK).into();
                let (rook_start_square, rook_end_square) = if move_to_undo.end_square % 16 < 4 {
                    (move_to_undo.end_square - 2, move_to_undo.end_square + 1)
                }
                else {
                    (move_to_undo.end_square + 1, move_to_undo.end_square - 1)
                };
                
                self.set_piece(move_to_undo.start_square, move_to_undo.moved_piece);
                self.set_piece(move_to_undo.end_square, Empty);
                self.set_piece(rook_start_square, rook);
                self.set_piece(rook_end_square, Empty);
                
                replace_vec(self.get_piece_position_mut(move_to_undo.moved_piece), move_to_undo.end_square, move_to_undo.start_square);
                replace_vec(self.get_piece_position_mut(rook), rook_end_square, rook_start_square);
                
            }
            _ => {
                
                self.set_piece(move_to_undo.start_square, move_to_undo.moved_piece);
                self.set_piece(move_to_undo.end_square, move_to_undo.replaced_piece);
            
                replace_vec(self.get_piece_position_mut(move_to_undo.moved_piece), move_to_undo.end_square, move_to_undo.start_square);
                
                if move_to_undo.replaced_piece != Empty {
                    self.get_piece_position_mut(move_to_undo.replaced_piece).push(move_to_undo.end_square);
                }
            }
        }

        match move_to_undo.moved_piece {
            WhiteKing => self.white_king = move_to_undo.start_square,
            BlackKing => self.black_king = move_to_undo.start_square,
            _ => {}
        }

        self.en_passant_chance = self.previous_moves.last().and_then(|prev_move| {
            (prev_move.move_type == MoveType::PawnDouble).then(|| move_colour.offset_rank(prev_move.end_square))
        });
        
        self.castling_rights = move_to_undo.old_castling_rights;
        self.side_to_move = move_colour;
        self.turns_taken -= 1;

    }
}
