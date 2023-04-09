use super::options::SearchOptions;
use cozy_chess::{Board, Color};

pub fn timeman(option: SearchOptions, board: &Board) -> u32 {
    if let SearchOptions::Time(wtime, btime, _, _, _) = option {
        match board.side_to_move() {
            Color::White => wtime / 10,
            Color::Black => btime / 10,
        }
    } else {
        unreachable!();
    }
}
