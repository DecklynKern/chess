use super::chess_util::*;
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

#[derive(Clone, Copy)]
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
        let end_square = an_to_square(long_an.to_string()[2..4].to_string());
        let diff = start_square.max(end_square) - start_square.min(end_square);
        let piece = board.get_piece(start_square);

        if piece.is_pawn() {
            if diff == 32 {
                Self::new_pawn_double(board, start_square, end_square)
            }
            else if diff != 16 {
                Self::new_en_passant(board, start_square, end_square)
            }
            else if !(16..=112).contains(&end_square) { // weirdest linting suggestion i've ever seen
                Self::new_promotion(board, start_square, end_square, Piece::from_char(long_an.to_string().chars().collect::<Vec<char>>()[5]))
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

    pub fn to_long_an(&self) -> String {
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

    pub fn to_an(&self, possible_moves: &[Move]) -> String {

        let moved_piece = self.moved_piece;
        let mut same_dest_moves: Vec<&Move> = Vec::new();

        for possible_move in possible_moves {
            if possible_move.moved_piece == moved_piece && possible_move.end_square == self.end_square && possible_move.start_square != self.start_square {
                same_dest_moves.push(possible_move);
            }
        }

        if self.move_type == MoveType::Castle {
            return String::from(match self.end_square {
                C1 | C8 => "O-O-O",
                G1 | G8 => "O-O",
                _ => unreachable!()
            })
        }

        return format!("{}{}{}{}{}",
            if moved_piece.is_pawn() {
                String::new()
            }
            else {
                moved_piece.to_an_char().to_string()
            },
            if same_dest_moves.is_empty() {
                String::new()
            }
            else {
                square_to_an(self.start_square)
            },
            if self.replaced_piece != Empty || self.move_type == MoveType::EnPassant {"x"} else {""},
            square_to_an(self.end_square),
            if let MoveType::Promotion(promote_piece) = self.move_type {
                format!("={}", promote_piece.to_an_char())
            }
            else {
                String::new()
            }
        );
    }
}
