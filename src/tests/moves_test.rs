#[cfg(test)]
mod tests {

    use crate::core::structs::Color as Color;
    use crate::core::structs::Square as Square;
    use crate::game::board::Board;
    use crate::game::piece::Piece as Piece;
    use crate::game::moves::Move as Move;

    
    #[test]
    fn test_from_uci() {
        let board1 = Board::from_fen("rnbqkbnr/pppppppp/8/8/4p3/8/PPPPPPPP/RNBQKBNR b KQkq e3 0 0").unwrap();
        let pawna2_a4 = Move::from_uci(&board1, "a2a4");
            assert_eq!(pawna2_a4.color, Color::White);
            assert_eq!(pawna2_a4.destination, Square::A4);
            assert_eq!(pawna2_a4.origin, Square::A2);
            assert_eq!(pawna2_a4.is_castle, false);
            assert_eq!(pawna2_a4.promote_type, None);
            assert_eq!(pawna2_a4.piece, Piece::Pawn);
    }

    /*
    #[test]
    fn test_generate_all_knight_moves() {
        assert_eq!(Move::generate_all_knight_moves(&, &Square::A2).len(), 4);
    }
     */

    fn step_depth(boards: Vec<Board>) -> Vec<Board>{
        let mut new_boards = Vec::new();
        for board in boards {
            let moves = Move::generate_legal_moves(&board);
            for turn in moves {
                let mut new_board = board.clone();
                new_board.process_move(&turn);
                new_boards.push(new_board);              
            }
        }
        new_boards
    }

    #[test]
    fn test_perft() {
        let boards_0 = vec![Board::from_fen("r3k2r/p1ppqpb1/bn2pnp1/3PN3/1p2P3/2N2Q1p/PPPBBPPP/R3K2R w KQkq - 0 1").unwrap()];
        let boards_1 = step_depth(boards_0);
        for board in boards_1 {
            board.print_board();
        }
        // assert_eq!(boards_1.len(), 48);
        /*
        let boards_2 = step_depth(boards_1);
        println!("{:?}", boards_2);
        assert_eq!(boards_2.len(), 191);
        let boards_3 = step_depth(boards_2);
        assert_eq!(boards_3.len(), 2812);
        let boards_4 = step_depth(boards_3);
        assert_eq!(boards_4.len(), 43238);
         */

    }

}