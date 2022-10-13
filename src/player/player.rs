use crate::simulator::board;
use crate::simulator::eval;

pub const MIN_SCORE: isize = isize::MIN + 1;
pub const MAX_SCORE: isize = isize::MAX;

pub trait Player {
    fn get_move<'a>(&mut self, board: &mut board::Board, possible_moves: &'a Vec<eval::Move>) -> Option<&'a eval::Move>;
}