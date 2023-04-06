use super::definitions::MAX_PLY;
use super::parse_move::cozymove_to_uci;
use cozy_chess::{Board, Move};

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

    // Converts the internal cozy-chess move to a UCI compliant move,
    // puts the move first and copies the old content behind the new move.
    pub fn store(&mut self, board: &Board, mv: Move, old: &Self) {
        let mv = cozymove_to_uci(&board, mv);
        self.table[0] = Some(mv);
        self.table[1..=old.length].copy_from_slice(&old.table[..old.length]);
        self.length = old.length + 1;
    }

    // For UCI output
    pub fn moves(&self) -> &[Option<Move>] {
        &self.table[..self.length]
    }

    // For UCI output
    pub fn pv_string(&self) -> String {
        let mut pv = String::new();
        for &mv in self.moves() {
            pv.push(' ');
            pv.push_str(mv.unwrap().to_string().as_str());
        }

        pv
    }
}
