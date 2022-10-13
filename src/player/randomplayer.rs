use crate::player::player;
use rand::seq::SliceRandom;
use rand::thread_rng;
use crate::simulator::board;
use crate::simulator::eval;

pub struct RandomPlayer {
}

impl player::Player for RandomPlayer {
    fn get_move<'a>(&mut self, _board: &mut board::Board, possible_moves: &'a Vec<eval::Move>) -> Option<&'a eval::Move> {
        possible_moves.choose(&mut thread_rng())
    }
}