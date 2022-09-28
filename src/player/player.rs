use crate::simulator::board;
use crate::simulator::eval;

pub const MIN_SCORE: isize = -10000000;
pub const MAX_SCORE: isize = 10000000;

pub trait Player {
    fn get_move<'a>(&mut self, board: &mut board::Board, possible_moves: &'a Vec<eval::Move>) -> &'a eval::Move;
}