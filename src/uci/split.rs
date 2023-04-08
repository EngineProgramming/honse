use std::str::SplitAsciiWhitespace;

pub fn split(stream: &mut SplitAsciiWhitespace, board: &mut cozy_chess::Board) {
    let depth = match stream.next().map(|x| x.parse::<u8>()) {
        Some(Ok(0)) => return,
        Some(Ok(depth)) => depth,
        Some(Err(_)) => return,
        None => return,
    };

    let mut total = 0;
    board.generate_moves(|moves| {
        for mv in moves {
            let mut board = board.clone();
            board.play_unchecked(mv);
            let nodes = crate::chess::perft::perft(&board, depth - 1);
            total += nodes;
            println!("{mv} {nodes}");
        }
        false
    });
    println!("nodes {total}");
}
