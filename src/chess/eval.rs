use cozy_chess::{Board, Color, Piece};

const PIECE_VALUES: [i16; 6] = [100, 320, 330, 500, 900, 0];

fn piece_type(piece: Piece) -> usize {
    match piece {
        Piece::Pawn => 0,
        Piece::Knight => 1,
        Piece::Bishop => 2,
        Piece::Rook => 3,
        Piece::Queen => 4,
        Piece::King => 5,
    }
}

pub fn eval(board: &Board) -> i16 {
    let mut white_score = 0;
    let mut black_score = 0;

    // Loops through each piece type
    for pt in Piece::ALL {
        // Loops through each square containing a piece of the current type
        for square in board.pieces(pt) {
            let piece = board.piece_on(square).unwrap();
            let color = board.color_on(square).unwrap();

            match color {
                Color::White => {
                    white_score += PIECE_VALUES[piece_type(piece)];
                }
                Color::Black => {
                    black_score += PIECE_VALUES[piece_type(piece)];
                }
            }
        }
    }

    if board.side_to_move() == Color::White {
        white_score - black_score
    } else {
        -(white_score - black_score)
    }
}
