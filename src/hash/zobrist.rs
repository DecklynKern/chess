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
    white_pawn: [u64; 64],
    white_knight: [u64; 64],
    white_bishop: [u64; 64],
    white_rook: [u64; 64],
    white_queen: [u64; 64],
    white_king: [u64; 64],
    black_pawn: [u64; 64],
    black_knight: [u64; 64],
    black_bishop: [u64; 64],
    black_rook: [u64; 64],
    black_queen: [u64; 64],
    black_king: [u64; 64],
    side_to_move_is_black: u64,
    castling_rights: [u64; 16],
    en_passant_file: [u64; 9]
}

impl Zobrist {

    pub fn new() -> Zobrist {

        let mut rng = RNG::new();

        Zobrist{
            white_pawn: array_init::array_init(|_| rng.get_rand()),
            white_knight: array_init::array_init(|_| rng.get_rand()),
            white_bishop: array_init::array_init(|_| rng.get_rand()),
            white_rook: array_init::array_init(|_| rng.get_rand()),
            white_queen: array_init::array_init(|_| rng.get_rand()),
            white_king: array_init::array_init(|_| rng.get_rand()),
            black_pawn: array_init::array_init(|_| rng.get_rand()),
            black_knight: array_init::array_init(|_| rng.get_rand()),
            black_bishop: array_init::array_init(|_| rng.get_rand()),
            black_rook: array_init::array_init(|_| rng.get_rand()),
            black_queen: array_init::array_init(|_| rng.get_rand()),
            black_king: array_init::array_init(|_| rng.get_rand()),
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

            hash ^= match board.get_piece_abs(pos) {
                piece::WHITE_PAWN => self.white_pawn[idx],
                piece::WHITE_KNIGHT => self.white_knight[idx],
                piece::WHITE_BISHOP => self.white_bishop[idx],
                piece::WHITE_ROOK => self.white_rook[idx],
                piece::WHITE_QUEEN => self.white_queen[idx],
                piece::WHITE_KING => self.white_king[idx],
                piece::BLACK_PAWN => self.black_pawn[idx],
                piece::BLACK_KNIGHT => self.black_knight[idx],
                piece::BLACK_BISHOP => self.black_bishop[idx],
                piece::BLACK_ROOK => self.black_rook[idx],
                piece::BLACK_QUEEN => self.black_queen[idx],
                piece::BLACK_KING => self.black_king[idx],
                piece::EMPTY => 0,
                piece::BORDER => 0
            };

            idx += 1;

        }

        if board.side_to_move == piece::Colour::Black {
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