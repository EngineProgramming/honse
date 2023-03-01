use crate::search::options::SearchOptions;
use rand::seq::SliceRandom;
use std::str::SplitAsciiWhitespace;

#[derive(Debug)]
enum SearchOptionsError {
    UnrecognisedToken,
    InvalidCombination,
}

fn parse_go(stream: &mut SplitAsciiWhitespace) -> Result<SearchOptions, SearchOptionsError> {
    let mut wtime = None;
    let mut btime = None;
    let mut winc = None;
    let mut binc = None;
    let mut movestogo = None;
    let mut depth = None;
    let mut nodes = None;
    let mut movetime = None;
    let mut infinite = None;

    loop {
        match (
            stream.next().unwrap_or_default(),
            stream.next().unwrap_or_default(),
        ) {
            ("wtime", n) => wtime = n.parse::<u32>().ok(),
            ("btime", n) => btime = n.parse::<u32>().ok(),
            ("winc", n) => winc = n.parse::<u32>().ok(),
            ("binc", n) => binc = n.parse::<u32>().ok(),
            ("movestogo", n) => movestogo = n.parse::<u32>().ok(),
            ("depth", n) => depth = n.parse::<u8>().ok(),
            ("nodes", n) => nodes = n.parse::<u64>().ok(),
            ("movetime", n) => movetime = n.parse::<u32>().ok(),
            ("infinite", _) => infinite = Some(true),
            ("", _) => break,
            (_, _) => return Err(SearchOptionsError::UnrecognisedToken),
        }
    }

    match (wtime, btime, depth, nodes, movetime, infinite) {
        (Some(wt), Some(bt), None, None, None, None) => {
            Ok(SearchOptions::Time(wt, bt, winc, binc, movestogo))
        }
        (None, None, Some(d), None, None, None) => Ok(SearchOptions::Depth(d)),
        (None, None, None, Some(n), None, None) => Ok(SearchOptions::Nodes(n)),
        (None, None, None, None, Some(m), None) => Ok(SearchOptions::Movetime(m)),
        (None, None, None, None, None, Some(_)) => Ok(SearchOptions::Infinite),
        _ => Err(SearchOptionsError::InvalidCombination),
    }
}

pub fn go(stream: &mut SplitAsciiWhitespace, board: &mut cozy_chess::Board) {
    let opts = parse_go(stream);
    if opts.is_err() {
        return;
    }

    let mut legal_moves = vec![];
    board.generate_moves(|moves| {
        legal_moves.extend(moves);
        false
    });

    match legal_moves.choose(&mut rand::thread_rng()) {
        Some(mv) => println!("bestmove {mv}"),
        _ => println!("bestmove 0000"),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_success() {
        let tests: [(&str, SearchOptions); 10] = [
            (
                "wtime 123 btime 456",
                SearchOptions::Time(123, 456, None, None, None),
            ),
            (
                "btime 123 wtime 456",
                SearchOptions::Time(456, 123, None, None, None),
            ),
            (
                "wtime 123 btime 456 winc 4",
                SearchOptions::Time(123, 456, Some(4), None, None),
            ),
            (
                "wtime 123 btime 456 binc 5",
                SearchOptions::Time(123, 456, None, Some(5), None),
            ),
            (
                "wtime 123 btime 456 movestogo 6",
                SearchOptions::Time(123, 456, None, None, Some(6)),
            ),
            (
                "movestogo 6 btime 456 winc 7 wtime 123",
                SearchOptions::Time(123, 456, Some(7), None, Some(6)),
            ),
            ("movetime 123", SearchOptions::Movetime(123)),
            ("nodes 123", SearchOptions::Nodes(123)),
            ("depth 123", SearchOptions::Depth(123)),
            ("infinite", SearchOptions::Infinite),
        ];

        for (input, result) in tests {
            println!("{input}");
            let mut stream = input.split_ascii_whitespace();
            let options = parse_go(&mut stream);
            assert!(options.is_ok());
            assert_eq!(options.unwrap(), result);
        }
    }

    #[test]
    fn parse_failure() {
        let tests: [&str; 12] = [
            "",
            "test",
            "movetime",
            "movetime test",
            "wtime 123",
            "wtime 123 winc 123 binc 123",
            "movestogo 123",
            "infinite movetime 123",
            "winc 123 binc 456",
            "nodes infinite",
            "movetime 123 nodes 456",
            "wtime 123.0 btime 456",
        ];

        for input in tests {
            println!("{input}");
            let mut stream = input.split_ascii_whitespace();
            let gg = parse_go(&mut stream);
            assert!(gg.is_err());
        }
    }
}
