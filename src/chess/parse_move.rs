use cozy_chess::{Board, Move, MoveParseError, Piece, Square};

pub fn parse_move(board: &Board, movestr: &str) -> Result<Move, MoveParseError> {
    let mut mv: Move = movestr.parse()?;

    if board.piece_on(mv.from) == Some(Piece::King) && board.piece_on(mv.to) != Some(Piece::Rook) {
        mv.to = match (mv.from, mv.to) {
            (Square::E1, Square::G1) => Square::H1,
            (Square::E8, Square::G8) => Square::H8,
            (Square::E1, Square::C1) => Square::A1,
            (Square::E8, Square::C8) => Square::A8,
            _ => mv.to,
        };
    }

    Ok(mv)
}

pub fn move_to_string(board: &Board, mv: Move) -> String {
    if board.piece_on(mv.from) == Some(Piece::King) {
        return match (mv.from, mv.to) {
            (Square::E1, Square::H1) => String::from("e1g1"),
            (Square::E8, Square::H8) => String::from("e8g8"),
            (Square::E1, Square::A1) => String::from("e1c1"),
            (Square::E8, Square::A8) => String::from("e8c8"),
            _ => mv.to_string(),
        };
    }

    mv.to_string()
}

#[cfg(test)]
mod tests {
    use super::parse_move;
    use crate::chess::parse_move::move_to_string;
    use cozy_chess::{Board, Move};

    #[test]
    fn parse_white() {
        let tests: [(&str, &str); 5] = [
            ("e1g1", "e1h1"),
            ("e1c1", "e1a1"),
            ("e1h1", "e1h1"),
            ("e1a1", "e1a1"),
            ("e1e2", "e1e2"),
        ];

        let board: Board = "4k3/8/8/8/8/8/8/R3K2R w KQ - 0 1".parse().unwrap();

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

        let board: Board = "r3k2r/8/8/8/8/8/8/4K3 b kq - 0 1".parse().unwrap();

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

        let board: Board = "4k3/8/8/8/8/8/8/1R2K1R1 w GB - 0 1".parse().unwrap();

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
    fn move2string() {
        let tests: [(Move, &str); 7] = [
            ("e1h1".parse().unwrap(), "e1g1"),
            ("e1a1".parse().unwrap(), "e1c1"),
            ("e1g1".parse().unwrap(), "e1g1"),
            ("e1c1".parse().unwrap(), "e1c1"),
            ("e1e2".parse().unwrap(), "e1e2"),
            ("e2e4".parse().unwrap(), "e2e4"),
            ("d8d2".parse().unwrap(), "d8d2"),
        ];

        let board: Board = "4k3/8/8/8/8/8/8/R3K2R w KQ - 0 1".parse().unwrap();

        for (before, after) in tests {
            assert_eq!(move_to_string(&board, before), after);
        }
    }
}
