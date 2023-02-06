use crate::chess::constants::MAX_PLY;
use cozy_chess::Move;

pub struct PVTable {
    pub pv_length: [i16; MAX_PLY as usize],
    pub pv_table: [[Option<Move>; MAX_PLY as usize]; MAX_PLY as usize],
}

impl PVTable {
    pub fn new() -> Self {
        PVTable {
            pv_length: [0; MAX_PLY as usize],
            pv_table: [[None; MAX_PLY as usize]; MAX_PLY as usize],
        }
    }

    pub fn store_pv(&mut self, ply: i16, mv: Move) {
        let uply = ply as usize;
        self.pv_table[uply][uply] = Some(mv);

        for i in (uply + 1)..self.pv_length[uply + 1] as usize {
            self.pv_table[uply][i] = self.pv_table[uply + 1][i];
        }

        self.pv_length[uply] = self.pv_length[uply + 1];
    }

    pub fn parse(&self) -> String {
        let mut pv = String::new();
        for i in 0..self.pv_length[0] {
            if self.pv_table[0][i as usize].is_none() {
                break;
            }
            pv.push(' ');
            pv.push_str(&self.pv_table[0][i as usize].unwrap().to_string());
        }

        pv
    }
}
