use std::str::SplitAsciiWhitespace;

pub fn position(stream: &mut SplitAsciiWhitespace, board: &mut cozy_chess::Board) {
    // Parse startpos/fen
    match stream.next() {
        Some("startpos") => {
            *board = cozy_chess::Board::startpos();
            stream.next();
        }
        Some("fen") => {
            let fen: String = stream
                .take_while(|&part| part != "moves")
                .fold(String::new(), |a, b| a + b + " ");
            *board = fen.trim().parse().unwrap();
        }
        _ => {}
    }

    // Parse moves
    for x in stream.by_ref() {
        match crate::engine::parse_move::uci_to_cozymove(board, x) {
            Ok(mv) => board.play(mv),
            _ => break,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_startpos() {
        let tests: [(&str, &str); 3] = [
            (
                "",
                "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1",
            ),
            (
                "e2e4",
                "rnbqkbnr/pppppppp/8/8/4P3/8/PPPP1PPP/RNBQKBNR b KQkq e3 0 1",
            ),
            (
                "e2e4 c7c5 g1f3 d7d6 d2d4 c5d4 f3d4 g8f6 b1c3 g7g6",
                "rnbqkb1r/pp2pp1p/3p1np1/8/3NP3/2N5/PPP2PPP/R1BQKB1R w KQkq - 0 6",
            ),
        ];

        for (moves, fen_end) in tests {
            let input = format!("startpos moves {moves}");
            let mut stream = input.split_ascii_whitespace();
            let mut board = cozy_chess::Board::startpos();

            position(&mut stream, &mut board);

            let expected = fen_end.parse().unwrap();
            assert_eq!(board, expected);
        }
    }

    #[test]
    fn parse_fen() {
        let fens: [&str; 6] = [
            "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1",
            "r3k2r/p1ppqpb1/bn2pnp1/3PN3/1p2P3/2N2Q1p/PPPBBPPP/R3K2R w KQkq - 0 1",
            "rnbqkb1r/pp2pp1p/3p1np1/8/3NP3/2N5/PPP2PPP/R1BQKB1R w KQkq - 0 6",
            "rnbq1rk1/pp2ppbp/3p1np1/8/3NP3/2N1BP2/PPPQ2PP/R3KB1R b KQ - 2 8",
            "2rq1rk1/pp1bppbp/3p1np1/4n3/3NP3/1BN1BP2/PPPQ2PP/1K1R3R b - - 10 12",
            "2rqr1k1/pp1bppbp/3p1np1/4n3/3NP2P/1BN1BP2/PPPQ2P1/1K1R3R b - h3 0 13",
        ];

        for fen in fens {
            let input = format!("fen {fen}");
            let mut stream = input.split_ascii_whitespace();
            let mut board = cozy_chess::Board::startpos();

            position(&mut stream, &mut board);

            let expected = fen.parse().unwrap();
            assert_eq!(board, expected);
        }
    }

    #[test]
    fn parse_fen_moves() {
        let tests: [(&str, &str, &str); 5] = [
            (
                "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1",
                "",
                "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1",
            ),
            (
                "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1",
                "e2e4",
                "rnbqkbnr/pppppppp/8/8/4P3/8/PPPP1PPP/RNBQKBNR b KQkq e3 0 1",
            ),
            (
                "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1",
                "e2e4 c7c5 g1f3 d7d6 d2d4 c5d4 f3d4 g8f6 b1c3 g7g6",
                "rnbqkb1r/pp2pp1p/3p1np1/8/3NP3/2N5/PPP2PPP/R1BQKB1R w KQkq - 0 6",
            ),
            (
                "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1",
                "g2g3 g7g6 f1g2 f8g7 g1f3 g8f6 e1g1 e8g8",
                "rnbq1rk1/ppppppbp/5np1/8/8/5NP1/PPPPPPBP/RNBQ1RK1 w - - 6 5",
            ),
            (
                "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1",
                "g2g3 g7g6 f1g2 f8g7 g1f3 g8f6 e1h1 e8h8",
                "rnbq1rk1/ppppppbp/5np1/8/8/5NP1/PPPPPPBP/RNBQ1RK1 w - - 6 5",
            ),
        ];

        for (fen, moves, fen_end) in tests {
            let input = format!("fen {fen} moves {moves}");
            let mut stream = input.split_ascii_whitespace();
            let mut board = cozy_chess::Board::startpos();

            position(&mut stream, &mut board);

            let expected = fen_end.parse().unwrap();
            assert_eq!(board, expected);
        }
    }
}
