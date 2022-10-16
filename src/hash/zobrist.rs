use crate::game;
use std::num::Wrapping;
use array_init;

struct RNG {
    seed: Wrapping<i128>,
}

impl RNG {

    pub fn new() -> RNG {
        RNG{seed: Wrapping(0x24707328e71eb479975be17e82370742)}
    }

    pub fn get_rand(&mut self) -> u64 {
        self.seed *= 0xda942042e4dd58b5;
        return (self.seed >> 64).0 as u64;
    }

}

pub struct Zobrist {
    piece_positions: [[u64; 64]; 16],
    side_to_move_is_black: u64,
    castling_rights: [u64; 16],
    en_passant_file: [u64; 9]
}

impl Zobrist {

    pub fn new() -> Zobrist {

        let mut rng = RNG::new();

        Zobrist{
            piece_positions: array_init::array_init(|arr| 
                if (arr as usize) != (game::Empty as usize) {array_init::array_init(|_| rng.get_rand())}
                else {array_init::array_init(|_| 0)}),
            side_to_move_is_black: rng.get_rand(), 
            castling_rights: ::array_init::array_init(|_| rng.get_rand()),
            en_passant_file: ::array_init::array_init(|_| rng.get_rand())
        }
    }

    pub fn get_board_hash(&self, board: &game::Board) -> u64 {

        let mut hash = 0u64;

        let mut idx = 0;

        // pretty hacky but should be fast
        for pos in game::VALID_SQUARES {
            hash ^= self.piece_positions[board.get_piece_abs(pos) as usize][idx];
            idx += 1;
        }

        if board.side_to_move == game::Black {
            hash ^= self.side_to_move_is_black;
        }

        let mut castling_index = 0usize;
        castling_index += board.castling_rights.0 as usize;
        castling_index <<= 1;
        castling_index += board.castling_rights.1 as usize;
        castling_index <<= 1;
        castling_index += board.castling_rights.2 as usize;
        castling_index <<= 1;
        castling_index += board.castling_rights.3 as usize;
        hash ^= self.castling_rights[castling_index];

        hash ^= self.en_passant_file[match board.en_passant_chance {
            Some(square) => square % 12 - 1,
            None => 0
        }];

        return hash;

    }

    // nuked until i feel like touching this mess again

    /*

    pub fn update_hash(&self, mut hash: u64, move_made: game::Move) -> u64 {

        let moved_piece = self.piece_positions[move_made.moved_piece as usize];

        hash ^= moved_piece[move_made.start_square];
        hash ^= moved_piece[move_made.end_square];

        if move_made.replaced_piece != game::Empty {
            hash ^= self.piece_positions[move_made.replaced_piece as usize][move_made.end_square];
        }

        match (move_made.move_type, move_made.moved_piece.get_colour()) {
            (game::MoveType::EnPassant, game::White) => {
                hash ^= self.piece_positions[game::BlackPawn as usize][move_made.end_square + 12];
            },
            (game::MoveType::EnPassant, game::Black) => {
                hash ^= self.piece_positions[game::WhitePawn as usize][move_made.end_square - 12];
            },
            (game::MoveType::Castle, colour) => {
                let rook = self.piece_positions[(colour as u8 | game::ROOK) as usize];
                let (rook_start_square, rook_end_square) = if move_made.end_square % 12 < 6 {
                    (move_made.end_square - 2, move_made.end_square + 1)
                } else {
                    (move_made.end_square + 1, move_made.end_square - 1)
                };
                hash ^= rook[rook_start_square];
                hash ^= rook[rook_end_square];
            }
            (game::MoveType::Normal, _) => {},
        }

        hash ^= self.en_passant_file[match move_made.old_en_passant_chance {
            Some(square) => square % 12 - 1,
            None => 0
        }];

        match move_made.moved_piece {
            game::WhitePawn => {
                hash ^= self.en_passant_file[if move_made.start_square == move_made.end_square + 24 {
                    move_made.end_square % 12 - 1
                } else {
                    0
                }];
                if move_made.end_square < 36 {
                    hash ^= self.piece_positions[game::WhitePawn as usize][move_made.end_square];
                    hash ^= self.piece_positions[game::WhiteQueen as usize][move_made.end_square];
                }
            },
            game::BlackPawn => {
                hash ^= self.en_passant_file[if move_made.start_square == move_made.end_square - 24 {
                    move_made.end_square % 12 - 1
                } else {
                    0
                }];
                if move_made.end_square > 108 {
                    hash ^= self.piece_positions[game::BlackPawn as usize][move_made.end_square];
                    hash ^= self.piece_positions[game::BlackQueen as usize][move_made.end_square];
                }
            },
            _ => {}
        }

        // not done
        // not even sure if this would be any faster with all the branches

        /*
        match move_to_make.moved_piece {
            WhiteRook if move_to_make.start_square == 110 => self.castling_rights.1 = false,
            WhiteRook if move_to_make.start_square == 117 => self.castling_rights.0 = false,
            BlackRook if move_to_make.start_square == 26 => self.castling_rights.3 = false,
            BlackRook if move_to_make.start_square == 33 => self.castling_rights.2 = false,
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

        match (move_to_make.replaced_piece, move_to_make.end_square) {
            (WhiteRook, 110) => self.castling_rights.1 = false,
            (WhiteRook, 117) => self.castling_rights.0 = false,
            (BlackRook, 26) => self.castling_rights.3 = false,
            (BlackRook, 33) => self.castling_rights.2 = false,
            _ => {}
        }

        self.side_to_move = self.side_to_move.opposite();*/

        return hash;

    } */

}