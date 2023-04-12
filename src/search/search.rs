use super::{
    definitions::{INFINITY, MATE, MAX_PLY},
    eval::eval,
    options::SearchOptions,
    pv_table::PVTable,
};
use crate::{chess::move_gen, search::timeman::timeman};
use cozy_chess::{Board, GameStatus, Move};
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

    match board.status() {
        GameStatus::Won => return mated_in(ply),
        GameStatus::Drawn => return draw_score(),
        _ => (),
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

fn draw_score() -> i16 {
    0
}

fn mated_in(ply: u8) -> i16 {
    -MATE + ply as i16
}

fn mate_in(ply: u8) -> i16 {
    MATE - ply as i16
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn mate_in_1() {
        const TESTS: [(&str, &str); 3] = [
            (
                "4r2k/1p3rbp/2p1N1p1/p3n3/P2NB1nq/1P6/4R1P1/B1Q2RK1 b - - 4 32",
                "h4h2",
            ),
            (
                "4rb2/3qrk2/1p1p1n2/7p/P2P4/4R2P/1BQN1P2/1K4R1 w - - 3 39",
                "c2g6",
            ),
            (
                "r1bqkbnr/pp2pp1p/3p2p1/2p5/3nP3/2N3PP/PPP1NP2/R1BQKB1R b KQkq - 1 6",
                "d4f3",
            ),
        ];

        for (fen, mv) in TESTS.iter() {
            let mut info = SearchInfo::new();
            let mut pv = PVTable::new();

            let score = search(
                &mut info,
                -INFINITY,
                INFINITY,
                &fen.parse().unwrap(),
                3,
                0,
                &mut pv,
            );

            assert_eq!(score, mate_in(1));
            assert_eq!(pv.table[0], Some(mv.parse().unwrap()));
        }
    }

    #[test]
    fn mated_in_1() {
        const TESTS: [(&str, &str); 2] = [
            ("8/8/8/8/8/1k6/4r3/K7 w - - 0 1", "a1b1"),
            ("k7/1pR4R/8/Q7/8/8/8/7K b - - 0 1", "a8b8"),
        ];

        for (fen, mv) in TESTS.iter() {
            let mut info = SearchInfo::new();
            let mut pv = PVTable::new();

            let score = search(
                &mut info,
                -INFINITY,
                INFINITY,
                &fen.parse().unwrap(),
                3,
                0,
                &mut pv,
            );

            assert_eq!(score, mated_in(2));
            assert_eq!(pv.table[0], Some(mv.parse().unwrap()));
        }
    }

    #[test]
    fn mate_in_2() {
        const TESTS: [(&str, &str); 3] = [
            (
                "4r3/1pp2rbk/6pn/4n3/P3BN1q/1PB2bPP/8/2Q1RRK1 b - - 0 31",
                "h4g3",
            ),
            (
                "r2k1b1r/p1ppq2p/np3np1/5p2/1PPP4/P3PQ2/3N1PPP/R1B1K2R w KQ - 1 13",
                "f3a8",
            ),
            (
                "rn3r2/p2q1pBk/1p2p3/3pP3/P1bNnQ2/5NP1/1P3PBP/R3b1K1 w - - 1 19",
                "f4h6",
            ),
        ];

        for (fen, mv) in TESTS.iter() {
            let mut info = SearchInfo::new();
            let mut pv = PVTable::new();

            let score = search(
                &mut info,
                -INFINITY,
                INFINITY,
                &fen.parse().unwrap(),
                5,
                0,
                &mut pv,
            );

            assert_eq!(score, mate_in(3));
            assert_eq!(pv.table[0], Some(mv.parse().unwrap()));
        }
    }

    #[test]
    fn mated_in_2() {
        const TESTS: [(&str, &str); 2] = [
            (
                "r1bq1bkr/ppp3pp/2n5/3Qp3/2B5/8/PPPP1PPP/RNB1K2R b KQ - 0 8",
                "d8d5",
            ),
            (
                "rnb1k2r/pppp1ppp/8/2b5/3qP3/P1N5/1PP3PP/R1BQ1BKR w kq - 0 9",
                "d1d4",
            ),
        ];

        for (fen, mv) in TESTS.iter() {
            let mut info = SearchInfo::new();
            let mut pv = PVTable::new();

            let score = search(
                &mut info,
                -INFINITY,
                INFINITY,
                &fen.parse().unwrap(),
                5,
                0,
                &mut pv,
            );

            assert_eq!(score, mated_in(4));
            assert_eq!(pv.table[0], Some(mv.parse().unwrap()));
        }
    }

    #[test]
    fn draw_50mr() {
        const TESTS: [&str; 2] = [
            "7k/6n1/8/8/8/8/1N6/K7 w - - 99 6969",
            "nnn5/n1nn4/nnnn4/n1n5/7k/8/7K/8 w - - 99 6969",
        ];

        for fen in TESTS.iter() {
            let mut info = SearchInfo::new();
            let mut pv = PVTable::new();

            let score = search(
                &mut info,
                -INFINITY,
                INFINITY,
                &fen.parse().unwrap(),
                5,
                0,
                &mut pv,
            );

            assert_eq!(score, 0);
        }
    }
}
