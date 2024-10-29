use std::collections::HashMap;

use super::r#move::*;
use super::board::*;

/*
[Event "Live Chess"]
[Site "Chess.com"]
[Date "2024.10.09"]
[Round "-"]
[White "DeckSSB"]
[Black "fishmannan"]
[Result "1-0"]
[CurrentPosition "2q3k1/3n2p1/2N4p/r2nPp2/3P3P/3Q2P1/1P1B4/5RK1 b - -"]
[Timezone "UTC"]
[ECO "E05"]
[ECOUrl "https://www.chess.com/openings/Catalan-Opening-Open-Defense-Classical-Line-6.O-O-O-O"]
[UTCDate "2024.10.09"]
[UTCTime "23:33:13"]
[WhiteElo "1828"]
[BlackElo "1751"]
[TimeControl "900+10"]
[Termination "DeckSSB won by resignation"]
[StartTime "23:33:13"]
[EndDate "2024.10.10"]
[EndTime "00:02:13"]
[Link "https://www.chess.com/analysis/game/live/122255928737?tab=analysis&move=35"]
[WhiteUrl "https://images.chesscomfiles.com/uploads/v1/user/194942883.57840a5c.50x50o.761795ba1c62.png"]
[WhiteCountry "3"]
[WhiteTitle ""]
[BlackUrl "https://www.chess.com/bundles/web/images/noavatar_l.84a92436.gif"]
[BlackCountry "27"]
[BlackTitle ""]

1. d4 Nf6 2. c4 e6 3. g3 d5 4. Bg2 Be7 5. Nf3 O-O 6. O-O dxc4 7. Qc2 Nc6 $6 8.
Qxc4 a6 9. Rd1 Ne8 $2 10. e4 $9 Nd6 11. Qe2 Nb5 $6 12. Be3 Bf6 $6 13. e5 Be7 14. a4
Nba7 15. h4 f6 $6 16. Nc3 f5 $6 17. Ng5 h6 18. Nh3 Nb4 19. Qc4 $2 (19. d5 Qe8 (19...
exd5 20. Nxd5 Nxd5 21. Bxd5+ Kh7 22. Bg8+ (22. Bf7 Rxf7 23. Rxd8 Bxd8)) 20.
Rac1) 19... c6 20. Nf4 Nd5 21. Ng6 Rf7 22. Nxe7+ $2 Qxe7 $9 23. Bd2 f4 $2 24. Ne4 Bd7
25. Nd6 Rff8 26. Nxb7 Nc8 27. Nc5 Ncb6 28. Qd3 Be8 29. a5 Nd7 30. Nxa6 fxg3 31.
fxg3 Bh5 32. Rf1 Bg4 33. Rxf8+ Qxf8 34. Rf1 Bf5 35. Be4 Qc8 36. Bxf5 exf5 37.
Nb4 Rxa5 38. Nxc6 Ra6 39. Qc4 $4 (39. Nb4 Nxb4 40. Bxb4 f4 41. gxf4 Rc6 42. f5
Nb6 43. Bc3) 1-0 */

type MoveIndex = std::num::NonZeroUsize;

#[derive(Clone, Copy, Debug)]
pub enum GameResult {
    Loss,
    Draw,
    Win
}

#[derive(Debug)]
pub struct MoveNode {
    played_move: Move,
    main_line: Option<MoveIndex>,
    alternatives: Vec<MoveIndex>
}

#[derive(Debug)]
pub struct Game {
    pub result: GameResult,
    pub move_list: Vec<MoveNode>
}

impl Game {

    fn parse_line<'a>(text: &mut impl Iterator<Item = &'a str>, board: &mut Board, move_list: &mut Vec<MoveNode>) -> Option<MoveIndex> {

        let mut last_move_idx: Option<MoveIndex> = None;
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
                    move_list[last_move_idx.unwrap().get()].alternatives.push(alternative_move);
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
                move_list[idx.get()].main_line = MoveIndex::new(new_move_idx);
            }

            last_move_idx = MoveIndex::new(new_move_idx);

        }

        for _ in 0..num_moves {
            board.undo_move();
        }

        last_move_idx

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

        println!("{moves_text}");

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