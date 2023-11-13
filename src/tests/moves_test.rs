#[cfg(test)]
mod tests {

    use crate::core::structs::Color as Color;
    use crate::core::structs::Square as Square;
    use crate::game::board::Board;
    use crate::game::piece::Piece as Piece;
    use crate::game::movegen::moves::Move as Move;
    use std::collections::HashMap as HashMap;

    
    #[test]
    fn test_from_uci() {
        let board1 = Board::from_fen("rnbqkbnr/pppppppp/8/8/4p3/8/PPPPPPPP/RNBQKBNR b KQkq e3 0 0").unwrap();
        let pawna2_a4 = Move::from_uci(&board1, "a2a4");
            assert_eq!(pawna2_a4.color, Color::White);
            assert_eq!(pawna2_a4.destination, Square::A4);
            assert_eq!(pawna2_a4.origin, Square::A2);
            assert!(!pawna2_a4.is_castle);
            assert_eq!(pawna2_a4.promote_type, None);
            assert_eq!(pawna2_a4.piece, Piece::Pawn);
    }

    fn print_step_depth(boards: Vec<Board>) -> Vec<Board>{
        let mut new_boards = Vec::new();
        let mut divide: HashMap<String, usize> = HashMap::new();
        for board in boards {
            let moves = Move::generate_legal_moves(&board);
            for turn in moves {
                let mut new_board = board;
                new_board.process_move(&turn);
                new_boards.push(new_board);    
                let key = turn.origin.to_str() + turn.destination.to_str().as_str();
                match divide.get(&key) {
                    Some(count) => { divide.insert(key, count + 1); }
                    None => { divide.insert(key, 1); }
                }
            }
        }
        for (key, value) in &divide {
            println!("{}: {}", key, value);
        }
        new_boards
    }

    fn step_depth(boards: Vec<Board>) -> Vec<Board>{
        let mut new_boards = Vec::new();
        for board in boards {
            let moves = Move::generate_legal_moves(&board);
            for turn in moves {
                let mut new_board = board;
                new_board.process_move(&turn);
                new_boards.push(new_board);             
            }
        }
        new_boards
    }

    #[test]
    fn test_perft() {
        
        
        let board1_0 = vec![Board::from_fen("r3k2r/Pppp1ppp/1b3nbN/nP6/BBP1P3/q4N2/Pp1P2PP/R2Q1RK1 w kq - 0 1").unwrap()];
        let board1_1 = step_depth(board1_0);
        assert_eq!(board1_1.len(), 6);
        let board1_2 = step_depth(board1_1);
        assert_eq!(board1_2.len(), 264);
        let board1_3 = step_depth(board1_2);
        assert_eq!(board1_3.len(), 9467);
        let board1_4 = step_depth(board1_3);
        assert_eq!(board1_4.len(), 422333);    
        // let board1_5 = step_depth(board1_4);
        // assert_eq!(board1_5.len(), 15833292);        
        
        
        
        // Starting position.
        let board2_0 = vec![Board::from_fen("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1").unwrap()];
        let board2_1 = step_depth(board2_0);
        assert_eq!(board2_1.len(), 20);
        let board2_2 = step_depth(board2_1);
        assert_eq!(board2_2.len(), 400);
        let board2_3 = step_depth(board2_2);
        assert_eq!(board2_3.len(), 8902);
        let board2_4 = step_depth(board2_3);
        assert_eq!(board2_4.len(), 197281);    
        // let board2_5 = step_depth(board2_4);
        // assert_eq!(board2_5.len(), 4865609);
        
        

        // Kiwipete.
        let board3_0 = vec![Board::from_fen("r3k2r/p1ppqpb1/bn2pnp1/3PN3/1p2P3/2N2Q1p/PPPBBPPP/R3K2R w KQkq - 0 1").unwrap()];
        let board3_1 = step_depth(board3_0);
        assert_eq!(board3_1.len(), 48);
        let board3_2 = step_depth(board3_1);
        assert_eq!(board3_2.len(), 2039);
        let board3_3 = step_depth(board3_2);
        assert_eq!(board3_3.len(), 97862);
        //let board3_4 = step_depth(board3_3);
        //assert_eq!(board3_4.len(), 4085603);  
        
        
        let board4_0 = vec![Board::from_fen("r3k2r/Pppp1ppp/1b3nbN/nP6/BBP1P3/q4N2/Pp1P2PP/R2Q1RK1 w kq - 0 1").unwrap()];
        let board4_1 = step_depth(board4_0);
        assert_eq!(board4_1.len(), 6);
        let board4_2 = step_depth(board4_1);
        assert_eq!(board4_2.len(), 264);
        let board4_3 = step_depth(board4_2);
        assert_eq!(board4_3.len(), 9467);
        let board4_4 = step_depth(board4_3);
        assert_eq!(board4_4.len(), 422333);  

        let board5_0 = vec![Board::from_fen("rnbq1k1r/pp1Pbppp/2p5/8/2B5/8/PPP1NnPP/RNBQK2R w KQ - 1 8").unwrap()];
        let board5_1 = step_depth(board5_0);
        assert_eq!(board5_1.len(), 44);
        let board5_2 = step_depth(board5_1);
        assert_eq!(board5_2.len(), 1486);
        let board5_3 = step_depth(board5_2);
        assert_eq!(board5_3.len(), 62379);
        let board5_4 = step_depth(board5_3);
        assert_eq!(board5_4.len(), 2103487);  
        
    }

}