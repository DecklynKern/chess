use crate::player::player;
use crate::simulator::board;
use crate::simulator::piece;
use crate::simulator::eval;
use crate::hash;
use crate::simulator::piece::Empty;

fn basic_move_ordering(moves: Vec<eval::Move>) -> Vec<eval::Move> {

    let mut sorted_moves = Vec::new();

    for possible_move in &moves {
        if possible_move.replaced_piece != Empty {
            sorted_moves.push(*possible_move);
        }
    }

    for possible_move in &moves {
        if possible_move.replaced_piece == Empty {
            sorted_moves.push(*possible_move);
        }
    }

    return sorted_moves;

}

pub struct AlphaBetaSearchPlayer {
    depth: usize,
    zobrist_hasher: hash::zobrist::Zobrist,
    transposition_table: hash::hashtable::HashTable<isize>,
    nodes_searched: usize
}

impl AlphaBetaSearchPlayer {

    pub fn new(depth: usize) -> AlphaBetaSearchPlayer {
        AlphaBetaSearchPlayer{
            depth: depth,
            zobrist_hasher: hash::zobrist::Zobrist::new(),
            transposition_table: hash::hashtable::HashTable::new(),
            nodes_searched: 0
        }
    }

    pub fn score_board(board: &board::Board) -> isize {
        let mut sum = 0;
        for square in board::VALID_SQUARES {
            let piece = board.get_piece_abs(square);
            sum += match piece {
                piece::WhitePawn => 100,
                piece::BlackPawn => -100,
                piece::WhiteKnight => 320,
                piece::BlackKnight => -320,
                piece::WhiteBishop => 330,
                piece::BlackBishop => -330,
                piece::WhiteRook => 530,
                piece::BlackRook => -530,
                piece::WhiteQueen => 960,
                piece::BlackQueen => -960,
                piece::WhiteKing => 0,
                piece::BlackKing => 0,
                piece::Empty => 0,
                piece::Border => 0
            };
        }
        return if board.side_to_move == piece::Colour::White {sum} else {-sum};
    }

    fn find_board_score(&mut self, board: &mut board::Board, depth: usize, mut alpha: isize, beta: isize) -> (isize, Option<eval::Move>) {

        self.nodes_searched += 1;

        let mut score: isize;

        if depth == 0 {
            score = AlphaBetaSearchPlayer::score_board(&board);
            let hash = self.zobrist_hasher.get_board_hash(board);
            self.transposition_table.set(hash, score);
            return (score, None);
        }

        score = player::MIN_SCORE;

        let possible_moves = eval::get_possible_moves(board);

        if possible_moves.is_empty() {
            
            if eval::get_king_attackers(board, board.side_to_move).is_empty() {
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

impl player::Player for AlphaBetaSearchPlayer {
    fn get_move<'a>(&mut self, board: &mut board::Board, possible_moves: &'a Vec<eval::Move>) -> Option<&'a eval::Move> {

        self.transposition_table.clear();

        let (score, best_move) = self.find_board_score(board, self.depth, player::MIN_SCORE, player::MAX_SCORE);

        println!("nodes searched: {}", self.nodes_searched);

        match best_move {
            Some(valid_move) => {
                for possible_move in possible_moves {
                    if possible_move.start_square == valid_move.start_square && possible_move.end_square == possible_move.end_square {
                        return Some(possible_move)
                    }
                }
                return None
            },
            None => return None
        }
    }
}