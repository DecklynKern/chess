use super::chess_util::*;
use super::piece::*;
use super::board::*;

#[derive(Clone, Copy, PartialEq)]
pub enum MoveType {
    Normal,
    PawnDouble,
    Promotion(Piece),
    EnPassant,
    Castle
}

#[derive(Clone)]
pub struct Move {
    pub start_square: usize,
    pub end_square: usize,
    pub moved_piece: Piece,
    pub replaced_piece: Piece,
    pub old_castling_rights : (bool, bool, bool, bool),
    pub move_type: MoveType
}

impl Move {

    fn create_move(board: &Board, start_square: usize, end_square: usize, move_type: MoveType) -> Move {
        let moved_piece = board.get_piece_abs(start_square);
        let replaced_piece = board.get_piece_abs(end_square);
        return Move {
            start_square,
            end_square,
            moved_piece,
            replaced_piece,
            old_castling_rights: board.castling_rights,
            move_type
        };
    }

    pub fn new(board: &Board, start_square: usize, end_square: usize) -> Move {
        Move::create_move(board, start_square, end_square, MoveType::Normal)
    }

    pub fn new_pawn_double(board: &Board, start_square: usize, end_square: usize) -> Move {
        Move::create_move(board, start_square, end_square, MoveType::PawnDouble)
    }

    pub fn new_en_passant(board: &Board, start_square: usize, end_square: usize) -> Move {
        Move::create_move(board, start_square, end_square, MoveType::EnPassant)
    }

    pub fn new_promotion(board: &Board, start_square: usize, end_square: usize, promote_piece: Piece) -> Move {
        Move::create_move(board, start_square, end_square, MoveType::Promotion(promote_piece))
    }

    pub fn new_castle(board: &Board, start_square: usize, end_square: usize) -> Move {
        Move::create_move(board, start_square, end_square, MoveType::Castle)
    }

    pub fn to_long_an(&self) -> String {
        format!("{}{}", index_to_long_an(self.start_square), index_to_long_an(self.end_square)) + match self.move_type {
            MoveType::Promotion(piece) => {
                if piece.is_knight() {
                    "n"
                } else if piece.is_bishop() {
                    "b"
                } else if piece.is_rook() {
                    "r"
                } else {
                    "q"
                }
            },
            _ => ""
        }
    }

    pub fn to_an(&self, possible_moves: &[Move]) -> String {

        let mut same_dest_moves: Vec<&Move> = Vec::new();

        for possible_move in possible_moves {
            if possible_move.moved_piece == self.moved_piece && possible_move.end_square == self.end_square && possible_move.start_square != self.start_square {
                same_dest_moves.push(possible_move);
            }

        }

        if self.move_type == MoveType::Castle {
            return String::from(match self.end_square {
                C8 => "o-o-o",
                G8 => "o-o",
                C1 => "O-O-O",
                G1 => "O-O",
                _ => unreachable!()
            })
        }

        return format!("{}{}{}{}",
            if self.moved_piece.is_pawn() {
                String::from("")
            } else {
                self.moved_piece.to_char().to_string()
            },
            if same_dest_moves.is_empty() {
                String::from("")
            } else {
                index_to_an(self.start_square)
            },
            if self.replaced_piece != Empty || self.move_type == MoveType::EnPassant {"x"} else {""},
            index_to_an(self.end_square)
        );
    }
}
