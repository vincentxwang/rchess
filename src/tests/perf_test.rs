extern crate test;

#[cfg(test)]
mod tests {
    use super::*;
    use test::Bencher;
    use crate::game::board::Board;
    use crate::game::movegen::moves::Move as Move;

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

    // The test below tests for performance of from_fen and move generation.
    #[bench]
    fn test_perft(b: &mut Bencher) {     

        b.iter(|| {
        // Starting position.
        let board2_0 = vec![Board::from_fen("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1").unwrap()];
        let board2_1 = step_depth(board2_0);
        assert_eq!(board2_1.len(), 20);
        let board2_2 = step_depth(board2_1);
        assert_eq!(board2_2.len(), 400);
        let board2_3 = step_depth(board2_2);
        assert_eq!(board2_3.len(), 8902);
        })
    }
}