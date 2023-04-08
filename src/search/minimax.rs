use super::{
    definitions::{INFINITY, MAX_PLY},
    eval::eval,
    options::SearchOptions,
    pv_table::PVTable,
};
use crate::chess::move_gen;
use cozy_chess::{Board, Color, Move};
use std::time::Instant;

pub struct Search {
    nodes: u64,
    start_timer: Option<Instant>,
    stop_time: Option<u32>,
    stop_flag: bool,
}

impl Search {
    pub fn new() -> Self {
        Search {
            nodes: 0,
            start_timer: None,
            stop_time: None,
            stop_flag: false,
        }
    }

    pub fn ab_search(
        &mut self,
        mut alpha: i16,
        beta: i16,
        board: &Board,
        depth: u8,
        ply: u8,
        pv: &mut PVTable,
    ) -> i16 {
        // Every 1024 nodes, check if we should stop
        if self.nodes % 1024 == 0 {
            if let (Some(timer), Some(stop_time)) = (self.start_timer, self.stop_time) {
                if timer.elapsed().as_millis() as u32 >= stop_time {
                    self.stop_flag = true;
                }
            }
        }

        // Always let depth 1 finish!
        if self.stop_flag && ply > 0 {
            return 0;
        }

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
            let score = -self.ab_search(-beta, -alpha, &new_board, depth - 1, ply + 1, &mut old_pv);

            if score <= best_score {
                continue;
            }
            best_score = score;

            if score <= alpha {
                continue;
            }
            // New best move
            alpha = score;
            pv.store(mv, &old_pv);

            // Fail-high
            if alpha >= beta {
                break;
            }
        }

        best_score
    }

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

    pub fn iterative_deepening(&mut self, board: &Board, option: SearchOptions, frc: bool) {
        let mut pv = PVTable::new();
        let mut best_move: Option<Move> = None;
        let info_timer = Instant::now();

        let depth = match option {
            SearchOptions::Depth(depth) => depth,
            SearchOptions::Time(_, _, _, _, _) => {
                self.start_timer = Some(Instant::now());
                self.stop_time = Some(Search::timeman(option, board));
                MAX_PLY
            }
            SearchOptions::Movetime(t) => {
                self.start_timer = Some(Instant::now());
                self.stop_time = Some(t);
                MAX_PLY
            }
            SearchOptions::Infinite | SearchOptions::Nodes(_) => MAX_PLY,
        };

        for d in 1..=depth {
            let score = self.ab_search(-INFINITY, INFINITY, board, d, 0, &mut pv);

            // Always clear at least depth 1
            // otherwise we might not have a best move
            if self.stop_flag && d > 1 {
                break;
            }

            let elapsed = info_timer.elapsed().as_millis() as u64;
            println!(
                "info depth {} score cp {} nodes {} nps {} time {} pv {}",
                d,
                score,
                self.nodes,
                self.nodes / (elapsed / 1000).max(1),
                elapsed,
                pv.to_string(board, frc)
            );

            best_move = pv.table[0];

            if let SearchOptions::Nodes(n) = option {
                if self.nodes >= n {
                    break;
                }
            }
        }

        println!("bestmove {}", best_move.unwrap());
    }

    pub fn reset(&mut self) {
        self.nodes = 0;
        self.start_timer = None;
        self.stop_flag = false;
    }
}
