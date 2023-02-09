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

#[cfg(test)]
mod tests {
    use super::*;
    use cozy_chess::{Board, Move, Square::*};

    #[test]
    fn ep_gen() {
        let board_1 = Board::from_fen(
            "rnbqkbnr/pppp1p1p/8/5Pp1/4p3/8/PPPPP1PP/RNBQKBNR w KQkq g6 0 4",
            false,
        )
        .unwrap();
        let moves_1 = capture_moves(&board_1);

        let board_2 = Board::from_fen(
            "rnbqkb1r/p1pppppp/5n2/Pp6/8/8/1PPPPPPP/RNBQKBNR w KQkq b6 0 3",
            false,
        )
        .unwrap();
        let moves_2 = capture_moves(&board_2);

        let board_3 = Board::from_fen(
            "rnbqkbnr/pppp1ppp/8/3P4/4pP2/8/PPP1P1PP/RNBQKBNR b KQkq f3 0 3",
            false,
        )
        .unwrap();
        let moves_3 = capture_moves(&board_3);

        assert!(moves_1.contains(&Move {
            from: F5,
            to: G6,
            promotion: None
        }));

        assert!(moves_2.contains(&Move {
            from: A5,
            to: B6,
            promotion: None
        }));

        assert!(moves_3.contains(&Move {
            from: E4,
            to: F3,
            promotion: None
        }));
    }
}
