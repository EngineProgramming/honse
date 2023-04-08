use super::definitions::MAX_PLY;
use crate::chess::parse_move::move_to_string;
use cozy_chess::{Board, Move};

#[derive(Clone)]
pub struct PVTable {
    pub length: usize,
    pub table: [Option<Move>; MAX_PLY as usize],
}

impl PVTable {
    pub fn new() -> Self {
        PVTable {
            length: 0,
            table: [None; MAX_PLY as usize],
        }
    }

    // Puts the new move first and copies the old content behind the new move.
    pub fn store(&mut self, mv: Move, old: &Self) {
        self.table[0] = Some(mv);
        self.table[1..=old.length].copy_from_slice(&old.table[..old.length]);
        self.length = old.length + 1;
    }

    pub fn moves(&self) -> &[Option<Move>] {
        &self.table[..self.length]
    }

    // For UCI output
    pub fn to_string(&self, board: &Board, frc: bool) -> String {
        let mut pv_string = String::new();
        // To keep track of the board state as we move through the PV line
        let mut new_board = board.clone();

        for &mv in self.moves() {
            let mv = mv.unwrap();

            // Internally, Cozy-Chess uses FRC notation.
            // So if we're not using FRC, we need to convert the move to standard notation.
            let mv_str = if frc {
                mv.to_string()
            } else {
                move_to_string(&new_board, mv)
            };

            pv_string.push_str(mv_str.as_str());
            pv_string.push(' ');

            new_board.play_unchecked(mv);
        }

        pv_string.trim_end().to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::PVTable;
    use cozy_chess::{Board, Move};

    #[test]
    fn pv_table() {
        let mut pv = PVTable::new();
        let board = Board::from_fen("r3k3/7P/8/4N3/5K2/8/8/8 b q - 1 1", false).unwrap();

        let mv: Move = "e8a8".parse().unwrap();
        pv.store(mv, &PVTable::new());

        let mv: Move = "f4f5".parse().unwrap();
        pv.store(mv, &pv.clone());

        // Standard
        assert_eq!("f4f5 e8c8", pv.to_string(&board, false));
        // FRC
        assert_eq!("f4f5 e8a8", pv.to_string(&board, true));
    }
}
