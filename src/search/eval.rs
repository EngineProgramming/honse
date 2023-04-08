use cozy_chess::{Board, Color, Piece};
const PIECE_VALUES: [i16; 6] = [100, 320, 330, 500, 900, 0];

pub fn eval(board: &Board) -> i16 {
    let mut score = 0;

    for piece in Piece::ALL {
        score +=
            PIECE_VALUES[piece as usize] * board.colored_pieces(Color::White, piece).len() as i16;

        score -=
            PIECE_VALUES[piece as usize] * board.colored_pieces(Color::Black, piece).len() as i16;
    }

    match board.side_to_move() {
        Color::White => score,
        Color::Black => -score,
    }
}

#[cfg(test)]
mod test {
    use cozy_chess::Board;

    use super::eval;

    #[test]
    fn eval_sanity() {
        let board = Board::from_fen(
            "rnb1kbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1",
            false,
        )
        .unwrap();

        assert!(eval(&board) > 0)
    }
}
