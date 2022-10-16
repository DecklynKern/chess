use crate::player::*;
use crate::game;
use crate::hash;

fn basic_move_ordering(moves: Vec<game::Move>) -> Vec<game::Move> {

    let mut sorted_moves = Vec::new();

    for possible_move in &moves {
        if possible_move.replaced_piece != game::Empty {
            sorted_moves.push(*possible_move);
        }
    }

    for possible_move in &moves {
        if possible_move.replaced_piece == game::Empty {
            sorted_moves.push(*possible_move);
        }
    }

    return sorted_moves;

}

pub struct AlphaBetaSearchPlayer {
    depth: usize,
    zobrist_hasher: hash::Zobrist,
    transposition_table: hash::HashTable<isize>,
    nodes_searched: usize
}

impl AlphaBetaSearchPlayer {

    pub fn new(depth: usize) -> AlphaBetaSearchPlayer {
        AlphaBetaSearchPlayer{
            depth: depth,
            zobrist_hasher: hash::Zobrist::new(),
            transposition_table: hash::HashTable::new(),
            nodes_searched: 0
        }
    }

    pub fn score_board(board: &game::Board) -> isize {
        let mut sum = 0;
        for square in game::VALID_SQUARES {
            let piece = board.get_piece_abs(square);
            sum += match piece {
                game::WhitePawn => 100,
                game::BlackPawn => -100,
                game::WhiteKnight => 320,
                game::BlackKnight => -320,
                game::WhiteBishop => 330,
                game::BlackBishop => -330,
                game::WhiteRook => 530,
                game::BlackRook => -530,
                game::WhiteQueen => 960,
                game::BlackQueen => -960,
                game::WhiteKing => 0,
                game::BlackKing => 0,
                game::Empty => 0,
                game::Border => 0
            };
        }
        return if board.side_to_move == game::Colour::White {sum} else {-sum};
    }

    fn find_board_score(&mut self, board: &mut game::Board, depth: usize, mut alpha: isize, beta: isize) -> (isize, Option<game::Move>) {

        self.nodes_searched += 1;

        let mut score: isize;

        if depth == 0 {
            score = AlphaBetaSearchPlayer::score_board(&board);
            let hash = self.zobrist_hasher.get_board_hash(board);
            self.transposition_table.set(hash, score);
            return (score, None);
        }

        score = MIN_SCORE;

        let possible_moves = game::get_possible_moves(board);

        if possible_moves.is_empty() {
            
            if game::get_king_attackers(board, board.side_to_move).is_empty() {
                return (0, None);
            }
            
            return (score, None);

        }

        let mut best_move = None;

        for possible_move in basic_move_ordering(possible_moves) {

            board.make_move(&possible_move);

            let hash = self.zobrist_hasher.get_board_hash(board);

            let move_score = match self.transposition_table.get(hash) {
                Some(cached_score) => cached_score,
                _ => {
                    let move_score = -self.find_board_score(board, depth - 1, -beta, -alpha).0;
                    self.transposition_table.set(hash, move_score);
                    move_score
                }
            };

            board.undo_move();

            if move_score > score {
                best_move = Some(possible_move);
                score = move_score;
            }

            alpha = alpha.max(score);

            if score >= beta {
                break;
            }
        }

        return (score, best_move);

    }
}

impl Player for AlphaBetaSearchPlayer {
    fn get_move<'a>(&mut self, board: &mut game::Board, possible_moves: &'a Vec<game::Move>) -> Option<&'a game::Move> {

        self.transposition_table.clear();

        let (_, best_move) = self.find_board_score(board, self.depth, MIN_SCORE, MAX_SCORE);

        // println!("nodes searched: {}", self.nodes_searched);

        match best_move {
            Some(valid_move) => {
                for possible_move in possible_moves {
                    if possible_move.start_square == valid_move.start_square && possible_move.end_square == valid_move.end_square {
                        return Some(possible_move)
                    }
                }
                return None
            },
            None => return None
        }
    }
}