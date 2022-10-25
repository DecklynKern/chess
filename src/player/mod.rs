mod humanplayer;
#[cfg(feature = "random")]
mod randomplayer;
mod minimaxsearchplayer;
mod alphabetasearchplayer;
mod scoring;

pub use humanplayer::*;
#[cfg(feature = "random")]
pub use randomplayer::*;
pub use minimaxsearchplayer::*;
pub use alphabetasearchplayer::*;
pub use scoring::*;

use crate::game;

pub trait Player {
    fn get_move<'a>(&mut self, board: &mut game::Board, possible_moves: &'a [game::Move]) -> Option<&'a game::Move>;
}