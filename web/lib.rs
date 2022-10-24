extern crate chess;
use wasm_bindgen::prelude::*;

static zobrist_hasher: chess::hash::Zobrist;
static hashtable: chess::hash::HashTable;
static player: Box<dyn game::player::Player>;

#[wasm_bindgen]
extern {
    pub fn chess_setup();
}

#[wasm_bindgen]
pub fn chess_setup() {
    zobrist_hasher = chess::hash::Zobrist::new();
    hashtable = chess::hash::HashTable::new();
    player = Box::new(game::player::AlphaBetaSearchPlayer::new(6));
}