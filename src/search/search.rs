use super::{
    definitions::{INFINITY, MAX_PLY},
    eval::eval,
    options::SearchOptions,
    pv_table::PVTable,
};
use crate::{chess::move_gen, search::timeman::timeman};
use cozy_chess::{Board, Move};
use std::time::Instant;

pub struct SearchInfo {
    pub nodes: u64,
    start_timer: Option<Instant>,
    stop_time: Option<u32>,
    stop_flag: bool,
}

impl SearchInfo {
    pub fn new() -> Self {
        SearchInfo {
            nodes: 0,
            start_timer: None,
            stop_time: None,
            stop_flag: false,
        }
    }

    pub fn reset(&mut self) {
        self.nodes = 0;
        self.start_timer = None;
        self.stop_time = None;
        self.stop_flag = false;
    }
}

pub fn search(
    info: &mut SearchInfo,
    mut alpha: i16,
    beta: i16,
    board: &Board,
    depth: u8,
    ply: u8,
    pv: &mut PVTable,
) -> i16 {
    // Every 1024 nodes, check if we should stop
    if info.nodes % 1024 == 0 {
        if let (Some(timer), Some(stop_time)) = (info.start_timer, info.stop_time) {
            if timer.elapsed().as_millis() as u32 >= stop_time {
                info.stop_flag = true;
            }
        }
    }

    // Always let depth 1 finish!
    if info.stop_flag && ply > 0 {
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
        info.nodes += 1;

        let score = -search(
            info,
            -beta,
            -alpha,
            &new_board,
            depth - 1,
            ply + 1,
            &mut old_pv,
        );

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

pub fn search_root(info: &mut SearchInfo, board: &Board, option: SearchOptions, frc: bool) {
    let mut pv = PVTable::new();
    let mut best_move: Option<Move> = None;
    let info_timer = Instant::now();

    let depth = match option {
        SearchOptions::Depth(depth) => depth,
        SearchOptions::Time(_, _, _, _, _) => {
            info.start_timer = Some(Instant::now());
            info.stop_time = Some(timeman(option, board));
            MAX_PLY
        }
        SearchOptions::Movetime(t) => {
            info.start_timer = Some(Instant::now());
            info.stop_time = Some(t);
            MAX_PLY
        }
        SearchOptions::Infinite | SearchOptions::Nodes(_) => MAX_PLY,
    };

    for d in 1..=depth {
        let score = search(info, -INFINITY, INFINITY, board, d, 0, &mut pv);

        // Always clear at least depth 1
        // otherwise we might not have a best move
        if info.stop_flag && d > 1 {
            break;
        }

        let elapsed = info_timer.elapsed().as_millis() as u64;
        println!(
            "info depth {} score cp {} nodes {} nps {} time {} pv {}",
            d,
            score,
            info.nodes,
            info.nodes / (elapsed / 1000).max(1),
            elapsed,
            pv.to_string(board, frc)
        );

        best_move = pv.table[0];

        if let SearchOptions::Nodes(n) = option {
            if info.nodes >= n {
                break;
            }
        }
    }

    println!("bestmove {}", best_move.unwrap());
}
