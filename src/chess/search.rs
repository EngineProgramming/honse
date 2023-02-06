use crate::chess::constants::{MAX_PLY, NEG_INF};
use crate::chess::eval::eval;
use crate::chess::move_gen;
use crate::chess::pv_table::PVTable;
use cozy_chess::{Board, Move};

struct Search {
    nodes: u64,
    pv_table: PVTable,
}

impl Search {
    pub fn new() -> Self {
        Search {
            nodes: 0,
            pv_table: PVTable::new(),
        }
    }

    pub fn search(&mut self, board: &Board, depth: u8, ply: i16) -> i16 {
        if ply >= MAX_PLY {
            return eval(board);
        }

        if depth == 0 {
            return eval(board);
        }

        let mut best_move: Option<Move> = None;
        let mut best_score = NEG_INF;

        for mv in move_gen::all_moves(board) {
            let mut new_board = board.clone();
            new_board.play_unchecked(mv);

            self.nodes += 1;

            let score = -self.search(&new_board, depth - 1, ply + 1);

            if score > best_score {
                best_score = score;
                self.pv_table.store_pv(ply, mv);
            }
        }

        best_score
    }
}