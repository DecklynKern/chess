mod humanplayer;
#[cfg(feature = "random")]
mod randommove;
mod minimax;
mod alphabeta;
mod iterativedeepening;
mod scoring;

pub use humanplayer::*;
#[cfg(feature = "random")]
pub use randommove::*;
pub use minimax::*;
pub use alphabeta::*;
pub use iterativedeepening::*;
pub use scoring::*;

use crate::game;

pub trait Player {
    fn get_move<'a>(&mut self, board: &mut game::Board, possible_moves: &'a [game::Move]) -> Option<&'a game::Move>;
}