use crate::player;
use crate::game::*;

use rand::seq::SliceRandom;
use rand::thread_rng;

pub struct RandomPlayer {
}

impl player::Player for RandomPlayer {
    fn get_move<'a>(&mut self, _board: &mut Board, possible_moves: &'a [Move]) -> Option<&'a Move> {
        possible_moves.choose(&mut thread_rng())
    }
}