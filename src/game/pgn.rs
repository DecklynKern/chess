use std::collections::HashMap;

use super::r#move::*;
use super::board::*;

type MoveIndex = std::num::NonZeroUsize;

#[derive(Clone, Copy, Debug)]
pub enum GameResult {
    Loss,
    Draw,
    Win
}

#[derive(Debug)]
pub struct MoveNode {
    pub played_move: Move,
    main_line: Option<MoveIndex>,
    alternatives: Vec<MoveIndex>
}

#[derive(Debug)]
pub struct Game {
    pub result: GameResult,
    move_list: Vec<MoveNode>
}

impl Game {

    pub fn get_root(&self) -> &MoveNode {
        &self.move_list[0]
    }

    pub fn get_main_line(&self, current_move: &MoveNode) -> Option<&MoveNode> {
        current_move.main_line.map(|idx| &self.move_list[idx.get()])
    }

    pub fn get_alternatives(&self, current_move: &MoveNode) -> Vec<&MoveNode> {
        current_move.alternatives.iter().map(|idx| &self.move_list[idx.get()]).collect()
    }

    fn parse_line<'a>(text: &mut impl Iterator<Item = &'a str>, board: &mut Board, move_list: &mut Vec<MoveNode>) -> Option<MoveIndex> {

        let mut last_move_idx: Option<usize> = None;
        let mut num_moves = 0;

        while let Some(mut chunk) = text.next() {

            chunk = chunk.trim();

            if chunk.is_empty() || chunk.ends_with('.') || chunk.starts_with('$') {
                continue;
            }

            if ["1-0", "1/2-1/2", "½-½", "0-1"].contains(&chunk) {
                break;
            }

            if chunk == "(" {

                let last_move = board.undo_move().unwrap();

                if let Some(alternative_move) = Self::parse_line(text, board, move_list) {
                    move_list[last_move_idx.unwrap()].alternatives.push(alternative_move);
                }

                board.make_move(&last_move);
                continue;

            }

            if chunk.ends_with(')') {
                break;
            }

            let played_move = Move::from_an(&chunk, board).unwrap();

            board.make_move(&played_move);
            num_moves += 1;

            move_list.push(MoveNode {
                played_move,
                main_line: None,
                alternatives: Vec::new()
            });

            let new_move_idx = move_list.len();

            if let Some(idx) = last_move_idx {
                move_list[idx].main_line = MoveIndex::new(new_move_idx);
            }

            last_move_idx = Some(new_move_idx);

        }

        for _ in 0..num_moves {
            board.undo_move();
        }

        last_move_idx.and_then(MoveIndex::new)

    }
    
    pub fn from_pgn(pgn: String) -> Self {
        
        let mut lines = pgn.split('\n');

        let mut tags = HashMap::new();

        while let Some(mut line) = lines.next() {

            if !line.starts_with('[') {
                break;
            }

            line = line.strip_prefix('[').unwrap();

            let mut quote_split = line.split("\"");

            let tag_name = quote_split.next().unwrap().trim();
            let tag_value = quote_split.next().unwrap().trim();

            tags.insert(tag_name, tag_value);

        }

        let moves_text = lines.fold(String::new(), |acc, line| acc + " " + line);
        let mut moves_split = moves_text.split_inclusive(&[' ', '(', ')']);

        let mut board = Board::default();
        let mut move_list = Vec::new();
        Self::parse_line(&mut moves_split, &mut board, &mut move_list);

        let result = match moves_split.next() {
            Some("1-0") => GameResult::Win,
            Some("0-1") => GameResult::Loss,
            _ => GameResult::Draw
        };

        Self {
            result,
            move_list
        }
    }
}
