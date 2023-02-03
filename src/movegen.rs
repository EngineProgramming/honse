use cozy_chess::{Board, Color, Move, Piece, Rank, Square};

pub fn all_moves(board: &Board) -> Vec<Move> {
    let mut move_list: Vec<Move> = Vec::new();

    board.generate_moves(|moves| {
        move_list.extend(moves);
        false
    });

    move_list
}

pub fn capture_moves(board: &Board) -> Vec<Move> {
    let enemy_pieces = board.colors(!board.side_to_move());
    let mut captures_list: Vec<Move> = Vec::new();

    // EP file
    let ep = board.en_passant();
    let mut ep_square: Option<Square> = None;

    if let Some(ep) = ep {
        if board.side_to_move() == Color::White {
            // En Passant is only possible on the 6th rank for white
            ep_square = Some(Square::new(ep, Rank::Sixth));
        } else {
            // Likewise but on the 3rd rank for black
            ep_square = Some(Square::new(ep, Rank::Third));
        }
    }

    board.generate_moves(|mut moves| {
        let mut permissible = enemy_pieces;
        if let Some(epsq) = ep_square {
            if moves.piece == Piece::Pawn {
                permissible |= epsq.bitboard();
            }
        }

        moves.to &= permissible;
        captures_list.extend(moves);
        false
    });

    captures_list
}
