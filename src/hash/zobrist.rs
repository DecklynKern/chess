use crate::simulator::piece;
use crate::simulator::board;
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
                if (arr as usize) != (piece::Empty as usize) {array_init::array_init(|_| rng.get_rand())}
                else {array_init::array_init(|_| 0)}),
            side_to_move_is_black: rng.get_rand(), 
            castling_rights: ::array_init::array_init(|_| rng.get_rand()),
            en_passant_file: ::array_init::array_init(|_| rng.get_rand())
        }
    }

    pub fn get_board_hash(&self, board: &board::Board) -> u64 {

        let mut hash = 0u64;

        let mut idx = 0;

        // pretty hacky but should be fast
        for pos in board::VALID_SQUARES {
            hash ^= self.piece_positions[board.get_piece_abs(pos) as usize][idx];
            idx += 1;
        }

        if board.side_to_move == piece::Black {
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

}