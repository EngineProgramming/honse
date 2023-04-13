use super::evaluation::psts::*;
use cozy_chess::{Board, Color};
use once_cell::sync::Lazy;

pub const PHASE_VALUES: [i32; 6] = [0, 1, 1, 2, 4, 0];
static PST: Lazy<PieceSquareTable> = Lazy::new(PieceSquareTable::new);

#[must_use]
pub fn eval(board: &Board) -> i16 {
    let mut mg = 0;
    let mut eg = 0;
    let mut game_phase = 0;

    for square in board.occupied() {
        let piece = board.piece_on(square).unwrap() as usize;
        let color = board.color_on(square).unwrap() as usize;
        let sq = square as usize;

        game_phase += PHASE_VALUES[piece];

        // PST contains material value.
        mg += PST.mg_pst[color + piece * 2][sq];
        eg += PST.eg_pst[color + piece * 2][sq];
    }

    let mg_weight = game_phase.min(24);
    let eg_weight = 24 - mg_weight;

    let score = ((mg * mg_weight) + (eg * eg_weight)) / 24;

    match board.side_to_move() {
        Color::White => score as i16,
        Color::Black => -score as i16,
    }
}

#[cfg(test)]
mod test {
    use super::eval;
    use cozy_chess::Board;

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
