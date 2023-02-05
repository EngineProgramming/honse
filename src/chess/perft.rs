#[allow(dead_code)]
pub fn perft(board: &cozy_chess::Board, depth: u8) -> u64 {
    if depth == 0 {
        1
    } else if depth == 1 {
        let mut nodes = 0;
        board.generate_moves(|moves| {
            nodes += moves.len() as u64;
            false
        });
        nodes
    } else {
        let mut nodes = 0;
        board.generate_moves(|moves| {
            for mv in moves {
                let mut board = board.clone();
                board.play_unchecked(mv);
                nodes += perft(&board, depth - 1);
            }
            false
        });
        nodes
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn perft_startpos() {
        let board = cozy_chess::Board::startpos();

        assert_eq!(perft(&board, 0), 1);
        assert_eq!(perft(&board, 1), 20);
        assert_eq!(perft(&board, 2), 400);
        assert_eq!(perft(&board, 3), 8902);
        assert_eq!(perft(&board, 4), 197281);
    }
}
