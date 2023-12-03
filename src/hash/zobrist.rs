use crate::game::{self, CastlingRights, Square};

struct RNG {
    seed: u128,
}

impl RNG {

    pub fn new() -> RNG {
        RNG {seed: 0x24707328e71eb479975be17e82370742}
    }

    pub fn get_rand(&mut self) -> u64 {
        self.seed = self.seed.wrapping_mul(0xda942042e4dd58b5);
        (self.seed >> 64) as u64
    }
}

pub struct Zobrist {
    piece_positions: [[u64; 144]; 16],
    side_to_move_is_black: u64,
    castling_rights: [u64; 16],
    en_passant_file: [u64; 8]
}

impl Zobrist {

    pub fn new() -> Zobrist {

        let mut rng = RNG::new();

        Zobrist {
            piece_positions: array_init::array_init(|arr| 
                if arr != (game::Empty as usize) {array_init::array_init(|_| rng.get_rand())}
                else {array_init::array_init(|_| 0)}),
            side_to_move_is_black: rng.get_rand(), 
            castling_rights: array_init::array_init(|_| rng.get_rand()),
            en_passant_file: array_init::array_init(|_| rng.get_rand())
        }
    }

    pub fn get_board_hash(&self, board: &game::Board) -> u64 {

        let mut hash = 0u64;

        for pos in game::VALID_SQUARES {
            hash ^= self.piece_positions[board.get_piece(pos) as usize][pos as usize];
        }

        if board.side_to_move == game::Black {
            hash ^= self.side_to_move_is_black;
        }

        hash ^= self.castling_rights[board.castling_rights as usize];

        hash ^= match board.en_passant_chance {
            Some(square) => self.en_passant_file[square as usize % 8],
            None => 0
        };

        return hash;

    }

    pub fn update_hash(&self, mut hash: u64, move_made: &game::Move, old_en_passant_chance: Option<u8>, old_castling_rights: game::CastlingRights, new_castling_rights: CastlingRights) -> u64 {

        let moved_piece_hashes = self.piece_positions[move_made.moved_piece as usize];
        
        let start = move_made.start_square as usize;
        let end = move_made.end_square as usize;

        hash ^= moved_piece_hashes[start];
        hash ^= moved_piece_hashes[end];

        if move_made.replaced_piece != game::Empty {
            hash ^= self.piece_positions[move_made.replaced_piece as usize][end];
        }

        let old_en_passant_hash = match old_en_passant_chance {
            Some(square) => self.en_passant_file[square as usize % 8],
            None => 0
        };
        hash ^= old_en_passant_hash;

        hash ^= match move_made.move_type {
            game::MoveType::EnPassant => 
                self.piece_positions[game::BlackPawn as usize][move_made.moved_piece.get_colour().opposite().offset_rank(end as Square) as usize],
            game::MoveType::PawnDouble =>
                old_en_passant_hash ^
                self.en_passant_file[end % 8],
            game::MoveType::Promotion(piece) =>
                moved_piece_hashes[end] ^ 
                self.piece_positions[piece as usize][end],
            game::MoveType::Castle => {
                
                let rook = self.piece_positions[(move_made.moved_piece.get_colour() as u8 | game::ROOK) as usize];
                let (rook_start_square, rook_end_square) = if end % 8 < 4 {
                    (end - 2, end + 1)
                } else {
                    (end + 1, end - 1)
                };
                
                rook[rook_start_square] ^ rook[rook_end_square]
                
            }
            game::MoveType::Normal => 0,
        };

        hash ^= self.castling_rights[old_castling_rights as usize];
        hash ^= self.castling_rights[new_castling_rights as usize];

        hash ^= self.side_to_move_is_black;

        return hash;

    }
}