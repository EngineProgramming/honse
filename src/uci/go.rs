use crate::search::{minimax::Search, options::SearchOption};
use cozy_chess::Board;
use std::str::SplitAsciiWhitespace;

#[derive(Debug)]
enum SearchOptionError {
    UnrecognisedToken,
    InvalidCombination,
}

fn parse_go(stream: &mut SplitAsciiWhitespace) -> Result<SearchOption, SearchOptionError> {
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
            (_, _) => return Err(SearchOptionError::UnrecognisedToken),
        }
    }

    match (wtime, btime, depth, nodes, movetime, infinite) {
        (Some(wt), Some(bt), None, None, None, None) => {
            Ok(SearchOption::Time(wt, bt, winc, binc, movestogo))
        }
        (None, None, Some(d), None, None, None) => Ok(SearchOption::Depth(d)),
        (None, None, None, Some(n), None, None) => Ok(SearchOption::Nodes(n)),
        (None, None, None, None, Some(m), None) => Ok(SearchOption::Movetime(m)),
        (None, None, None, None, None, Some(_)) => Ok(SearchOption::Infinite),
        _ => Err(SearchOptionError::InvalidCombination),
    }
}

pub fn go(stream: &mut SplitAsciiWhitespace, search: &mut Search, board: &Board) {
    let opts = parse_go(stream);

    if let Ok(opts) = opts {
        search.iterative_deepening(board, opts);
        search.reset();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_success() {
        let tests: [(&str, SearchOption); 10] = [
            (
                "wtime 123 btime 456",
                SearchOption::Time(123, 456, None, None, None),
            ),
            (
                "btime 123 wtime 456",
                SearchOption::Time(456, 123, None, None, None),
            ),
            (
                "wtime 123 btime 456 winc 4",
                SearchOption::Time(123, 456, Some(4), None, None),
            ),
            (
                "wtime 123 btime 456 binc 5",
                SearchOption::Time(123, 456, None, Some(5), None),
            ),
            (
                "wtime 123 btime 456 movestogo 6",
                SearchOption::Time(123, 456, None, None, Some(6)),
            ),
            (
                "movestogo 6 btime 456 winc 7 wtime 123",
                SearchOption::Time(123, 456, Some(7), None, Some(6)),
            ),
            ("movetime 123", SearchOption::Movetime(123)),
            ("nodes 123", SearchOption::Nodes(123)),
            ("depth 123", SearchOption::Depth(123)),
            ("infinite", SearchOption::Infinite),
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
