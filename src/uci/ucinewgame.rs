pub fn ucinewgame(board: &mut cozy_chess::Board) {
    *board = cozy_chess::Board::startpos();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn newgame() {
        let mut board = "rnbqkb1r/pp2pp1p/3p1np1/8/3NP3/2N5/PPP2PPP/R1BQKB1R w KQkq - 0 6"
            .parse()
            .unwrap();
        let expected = cozy_chess::Board::startpos();
        assert_ne!(board, expected);
        ucinewgame(&mut board);
        assert_eq!(board, expected);
    }
}
