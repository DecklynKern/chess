use super::chess_util::*;
use super::get_possible_moves;
use super::piece::*;
use super::board::*;

#[derive(Clone, Copy, PartialEq, Debug)]
pub enum MoveType {
    Normal,
    PawnDouble,
    Promotion(Piece),
    EnPassant,
    Castle
}

#[derive(Clone, Copy, Debug)]
pub struct Move {
    pub start_square: Square,
    pub end_square: Square,
    pub moved_piece: Piece,
    pub replaced_piece: Piece,
    pub old_castling_rights : CastlingRights,
    pub move_type: MoveType
}

impl Default for Move {
    fn default() -> Self {
        Self {
            start_square: 0,
            end_square: 0,
            moved_piece: Empty,
            replaced_piece: Empty,
            old_castling_rights: 0,
            move_type: MoveType::Normal
        }
    }
}

impl Move {

    fn create_move(board: &Board, start_square: Square, end_square: Square, move_type: MoveType) -> Self {
        
        let moved_piece = board.get_piece(start_square);
        let replaced_piece = board.get_piece(end_square);
        
        Self {
            start_square,
            end_square,
            replaced_piece,
            moved_piece,
            old_castling_rights: board.castling_rights,
            move_type
        }
    }

    pub fn new(board: &Board, start_square: Square, end_square: Square) -> Self {
        Self::create_move(board, start_square, end_square, MoveType::Normal)
    }

    pub fn new_pawn_double(board: &Board, start_square: Square, end_square: Square) -> Self {
        Self::create_move(board, start_square, end_square, MoveType::PawnDouble)
    }

    pub fn new_en_passant(board: &Board, start_square: Square, end_square: Square) -> Self {
        Self::create_move(board, start_square, end_square, MoveType::EnPassant)
    }

    pub fn new_promotion(board: &Board, start_square: Square, end_square: Square, promote_piece: Piece) -> Self {
        Self::create_move(board, start_square, end_square, MoveType::Promotion(promote_piece))
    }

    pub fn new_castle(board: &Board, start_square: Square, end_square: Square) -> Self {
        Move::create_move(board, start_square, end_square, MoveType::Castle)
    }

    pub fn from_long_an(long_an: &str, board: &Board) -> Self {

        let start_square = an_to_square(String::from(long_an));
        let end_square = an_to_square(long_an.to_string()[2..].trim().to_string());
        let diff = start_square.max(end_square) - start_square.min(end_square);
        let piece = board.get_piece(start_square);

        if piece.is_pawn() {
            if diff == 32 {
                Self::new_pawn_double(board, start_square, end_square)
            }
            else if diff != 16 && board.get_piece(end_square) == Piece::Empty {
                Self::new_en_passant(board, start_square, end_square)
            }
            else if !(16..=112).contains(&end_square) { // weirdest linting suggestion i've ever seen
                Self::new_promotion(board, start_square, end_square, Piece::from_char(long_an.to_string().chars().collect::<Vec<char>>()[4]))
            }
            else {
                Self::new(board, start_square, end_square) // necessary duplicate to cover all cases
            }
        }
        else if piece.is_king() && diff == 2 {
            Self::new_castle(board, start_square, end_square)
        }
        else {
            Self::new(board, start_square, end_square)
        }
    }

    pub fn from_an(an: &str, board: &Board) -> Option<Self> {

        let mut chars: Vec<_> = an.chars().collect();

        match chars.last() {
            None => return None,
            Some('+' | '#') => {
                chars.pop();
            }
            _ => {}
        }

        if chars == "O-O-O".chars().collect::<Vec<_>>() {
            if board.side_to_move == Colour::White {
                return Some(Self::new_castle(board, E1, C1));
            }
            else {
                return Some(Self::new_castle(board, E8, C8));
            }
        }
        else if chars == "O-O".chars().collect::<Vec<_>>() {
            if board.side_to_move == Colour::White {
                return Some(Self::new_castle(board, E1, G1));
            }
            else {
                return Some(Self::new_castle(board, E8, G8));
            }
        }

        let piece_code = if chars[0].is_uppercase() {
            match chars[0] {
                'N' => KNIGHT,
                'B' => BISHOP,
                'R' => ROOK,
                'Q' => QUEEN,
                'K' => KING,
                _ => return None
            }
        }
        else {
            PAWN
        };

        if piece_code != PAWN {
            chars.remove(0);
        }

        let moved_piece = Piece::from(piece_code | board.side_to_move as u8);

        let promotion_piece = if chars.contains(&'=') {
            
            let promotion_code = match chars.pop().unwrap() {
                'N' => KNIGHT,
                'B' => BISHOP,
                'R' => ROOK,
                'Q' => QUEEN,
                _ => return None
            };
            
            chars.pop();

            Some(Piece::from(promotion_code | board.side_to_move as u8))

        }
        else {
            None
        };

        let mut end_square_string = String::new();
        end_square_string.push(chars.pop().unwrap());
        end_square_string.insert(0, chars.pop().unwrap());

        let end_square = an_to_square(end_square_string);

        let takes = chars.contains(&'x');
        if takes {
            chars.pop();
        }

        let (disambiguate_file, disambiguate_rank) = match chars.len() {
            2 => (
                (Some((chars.pop().unwrap() as u8) - ('a' as u8))),
                (Some((chars.pop().unwrap() as u8) - ('0' as u8)))
            ),
            1 => {

                let char = chars.pop().unwrap();

                if char.is_numeric() {
                    (None, Some((char as u8) - ('0' as u8)))
                }
                else {
                    (Some((char as u8) - ('a' as u8)), None)
                }
            }
            _ => (None, None)
        };

        let legal_moves = get_possible_moves(board);

        for legal_move in legal_moves {

            if legal_move.end_square != end_square || legal_move.moved_piece != moved_piece {
                continue;
            }

            if let MoveType::Promotion(legal_promote_piece) = legal_move.move_type {
                if Some(legal_promote_piece) != promotion_piece {
                    continue;
                }
            }

            if let Some(rank) = disambiguate_rank {
                if rank != (legal_move.start_square) / 16 {
                    continue;
                }
            }

            if let Some(file) = disambiguate_file {
                if file != legal_move.start_square % 8 {
                    continue;
                }
            }

            return Some(legal_move)

        }

        None

    }

    pub fn as_long_an(&self) -> String {
        format!(
            "{}{}{}",
            square_to_an(self.start_square),
            square_to_an(self.end_square),
            if let MoveType::Promotion(piece) = self.move_type {
                piece.to_char().to_lowercase().to_string()
            }
            else {
                "".to_string()
            }
        )
    }

    pub fn as_an(&self, possible_moves: &[Move]) -> String {

        let moved_piece = self.moved_piece;

        let mut distinguish_file = false;
        let mut distinguish_rank = false;

        for possible_move in possible_moves {
            if possible_move.moved_piece == moved_piece && possible_move.end_square == self.end_square && possible_move.start_square != self.start_square {
                
                if possible_move.start_square % 8 == self.start_square % 8 {
                    distinguish_rank = true;
                }
                else if possible_move.start_square / 8 == self.start_square / 8 {
                    distinguish_file = true;
                }
            }
        }

        if self.move_type == MoveType::Castle {
            return String::from(match self.end_square {
                C1 | C8 => "O-O-O",
                G1 | G8 => "O-O",
                _ => unreachable!()
            })
        }

        let mut piece_name = if moved_piece.is_pawn() {
            String::new()
        }
        else {
            moved_piece.to_an_char().to_string()
        };

        let start_square_an = square_to_an(self.start_square);

        let takes = if self.replaced_piece != Empty || self.move_type == MoveType::EnPassant {
            if moved_piece.is_pawn() {
                piece_name = start_square_an.chars().nth(0).unwrap().to_string();
            }
            "x"
        }
        else {
            ""
        };

        let start_square = if distinguish_rank && distinguish_file {
            start_square_an
        }
        else if distinguish_rank {
            start_square_an.chars().nth(1).unwrap().to_string()
        }
        else if distinguish_file {
            start_square_an.chars().nth(0).unwrap().to_string()
        }
        else {
            String::new()
        };

        let promotion = if let MoveType::Promotion(promote_piece) = self.move_type {
            format!("={}", promote_piece.to_an_char())
        }
        else {
            String::new()
        };

        return format!("{}{}{}{}{}",
            piece_name,
            start_square,
            takes,
            square_to_an(self.end_square),
            promotion
        );
    }
}
