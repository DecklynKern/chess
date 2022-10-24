mod humanplayer;
#[cfg(feature = "random")]
mod randomplayer;
mod basicsearchplayer;
mod alphabetasearchplayer;

pub use humanplayer::*;
#[cfg(feature = "random")]
pub use randomplayer::*;
pub use basicsearchplayer::*;
pub use alphabetasearchplayer::*;

use crate::game;

pub const MIN_SCORE: i64 = i64::MIN + 1;
pub const MAX_SCORE: i64 = i64::MAX;

pub trait Player {
    fn get_move<'a>(&mut self, board: &mut game::Board, possible_moves: &'a Vec<game::Move>) -> Option<&'a game::Move>;
}