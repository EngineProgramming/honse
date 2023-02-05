pub fn parse_move(
    board: &cozy_chess::Board,
    movestr: &str,
) -> Result<cozy_chess::Move, cozy_chess::MoveParseError> {
    let mut mv: cozy_chess::Move = movestr.parse()?;

    if board.piece_on(mv.from) == Some(cozy_chess::Piece::King)
        && board.piece_on(mv.to) != Some(cozy_chess::Piece::Rook)
    {
        mv.to = match (mv.from, mv.to) {
            (cozy_chess::Square::E1, cozy_chess::Square::G1) => cozy_chess::Square::H1,
            (cozy_chess::Square::E8, cozy_chess::Square::G8) => cozy_chess::Square::H8,
            (cozy_chess::Square::E1, cozy_chess::Square::C1) => cozy_chess::Square::A1,
            (cozy_chess::Square::E8, cozy_chess::Square::C8) => cozy_chess::Square::A8,
            _ => mv.to,
        };
    }

    Ok(mv)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_white() {
        let tests: [(&str, &str); 5] = [
            ("e1g1", "e1h1"),
            ("e1c1", "e1a1"),
            ("e1h1", "e1h1"),
            ("e1a1", "e1a1"),
            ("e1e2", "e1e2"),
        ];

        let board: cozy_chess::Board = "4k3/8/8/8/8/8/8/R3K2R w KQ - 0 1".parse().unwrap();

        for (before, after) in tests {
            if let Ok(mv) = parse_move(&board, before) {
                assert_eq!(format!("{mv}"), after);
                assert!(board.is_legal(mv));
            } else {
                panic!("Failed to parse move {before}");
            }
        }
    }

    #[test]
    fn parse_black() {
        let tests: [(&str, &str); 5] = [
            ("e8g8", "e8h8"),
            ("e8c8", "e8a8"),
            ("e8h8", "e8h8"),
            ("e8a8", "e8a8"),
            ("e8e7", "e8e7"),
        ];

        let board: cozy_chess::Board = "r3k2r/8/8/8/8/8/8/4K3 b kq - 0 1".parse().unwrap();

        for (before, after) in tests {
            if let Ok(mv) = parse_move(&board, before) {
                assert_eq!(format!("{mv}"), after);
                assert!(board.is_legal(mv));
            } else {
                panic!("Failed to parse move {before}");
            }
        }
    }

    #[test]
    fn parse_960() {
        let tests: [(&str, &str); 2] = [("e1g1", "e1g1"), ("e1e2", "e1e2")];

        let board: cozy_chess::Board = "4k3/8/8/8/8/8/8/1R2K1R1 w GB - 0 1".parse().unwrap();

        for (before, after) in tests {
            if let Ok(mv) = parse_move(&board, before) {
                assert_eq!(format!("{mv}"), after);
                assert!(board.is_legal(mv));
            } else {
                panic!("Failed to parse move {before}");
            }
        }
    }
}
