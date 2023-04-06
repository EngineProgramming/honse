use std::str::SplitAsciiWhitespace;
use std::time::Instant;

pub fn perft(stream: &mut SplitAsciiWhitespace, board: &mut cozy_chess::Board) {
    let depth = match stream.next().map(|x| x.parse::<u8>()) {
        Some(Ok(depth)) => depth,
        Some(Err(_)) => return,
        None => return,
    };

    for i in 1..=depth {
        let start = Instant::now();
        let nodes = crate::engine::perft::perft(board, i);
        let duration = start.elapsed();
        let nps = nodes as f64 / duration.as_secs_f64();
        println!(
            "info depth {} nodes {} time {:?} nps {}",
            i,
            nodes,
            duration.as_millis(),
            nps as u64
        );

        if i == depth {
            println!("nodes {nodes}");
        }
    }
}
