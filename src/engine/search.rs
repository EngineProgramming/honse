use super::{
    definitions::{INFINITY, MAX_PLY},
    eval::eval,
    move_gen,
    pv_table::PVTable,
};
use cozy_chess::{Board, Move};

pub struct Search {
    pub nodes: u64,
}

impl Search {
    pub fn new() -> Self {
        Search { nodes: 0 }
    }

    pub fn search(
        &mut self,
        mut alpha: i16,
        beta: i16,
        board: &Board,
        depth: u8,
        ply: u8,
        pv: &mut PVTable,
    ) -> i16 {
        if ply >= MAX_PLY {
            return eval(board);
        }

        // The PVTable that will get passed down the search tree
        let mut old_pv = PVTable::new();
        // Our PV will get it's length from the old_pv
        pv.length = 0;

        if depth == 0 {
            return eval(board);
        }

        let mut best_score = -INFINITY;
        let moves: Vec<Move> = move_gen::all_moves(board);

        for mv in moves {
            let mut new_board = board.clone();
            new_board.play_unchecked(mv);
            self.nodes += 1;

            // Negamax
            let score = -self.search(-beta, -alpha, &new_board, depth - 1, ply + 1, &mut old_pv);

            if score <= best_score {
                continue;
            }
            best_score = score;

            if score <= alpha {
                continue;
            }
            // New best move
            alpha = score;
            pv.store(board, mv, &old_pv);

            // Fail-high
            if alpha >= beta {
                break;
            }
        }

        best_score
    }
}
