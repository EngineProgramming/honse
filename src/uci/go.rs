use crate::search::options::SearchOptions;
use rand::seq::SliceRandom;
use std::str::SplitAsciiWhitespace;

fn parse_go(stream: &mut SplitAsciiWhitespace) -> Result<SearchOptions, &'static str> {
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
        match (stream.next(), stream.next()) {
            (Some("wtime"), Some(n)) => wtime = n.parse::<u32>().ok(),
            (Some("btime"), Some(n)) => btime = n.parse::<u32>().ok(),
            (Some("winc"), Some(n)) => winc = n.parse::<u32>().ok(),
            (Some("binc"), Some(n)) => binc = n.parse::<u32>().ok(),
            (Some("movestogo"), Some(n)) => movestogo = n.parse::<u32>().ok(),
            (Some("depth"), Some(n)) => depth = n.parse::<u8>().ok(),
            (Some("nodes"), Some(n)) => nodes = n.parse::<u64>().ok(),
            (Some("movetime"), Some(n)) => movetime = n.parse::<u32>().ok(),
            (Some("infinite"), _) => infinite = Some(true),
            (Some(_), _) => {
                return Err("Uh oh");
            }
            _ => break,
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
        _ => Err("Uh oh"),
    }
}

pub fn go(stream: &mut SplitAsciiWhitespace, board: &mut cozy_chess::Board) {
    let _opts = parse_go(stream);

    let mut legal_moves = vec![];
    board.generate_moves(|moves| {
        for mv in moves {
            legal_moves.push(mv);
        }
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
            let gg = parse_go(&mut stream);
            assert!(gg.is_ok());
            assert_eq!(gg.unwrap(), result);
        }
    }

    #[test]
    fn parse_failure() {
        let tests: [&str; 10] = [
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
        ];

        for input in tests {
            println!("{input}");
            let mut stream = input.split_ascii_whitespace();
            let gg = parse_go(&mut stream);
            assert!(gg.is_err());
        }
    }
}
