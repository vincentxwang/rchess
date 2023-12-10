extern crate test;

#[cfg(test)]
mod tests {
    use super::*;
    use test::Bencher;
    use crate::core::structs::{Color, Square};
    use crate::engine::evaluate::Score;
    use crate::game::board::Board;
    use crate::game::movegen::moves::Move as Move;
    use crate::game::piece::Piece;

    fn step_depth(boards: &Vec<Board>) -> Vec<Board>{
        let mut new_boards = Vec::new();
        for board in boards {
            let moves = Move::generate_legal_moves(&board);
            for turn in moves {
                let mut new_board = *board;
                new_board.process_move(&turn).expect("generate_legal_moves() generated process_move() thinks is illegal. :/");
                new_boards.push(new_board);             
            }
        }
        new_boards
    }

    // Benches performance of from_fen and move generation.
    #[bench]
    fn test_perft(b: &mut Bencher) {     
        let board2_0 = vec![Board::from_fen("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1").unwrap()];

        b.iter(|| {
        // Starting position.
        let board2_1 = step_depth(&board2_0);
        assert_eq!(board2_1.len(), 20);
        let board2_2 = step_depth(&board2_1);
        assert_eq!(board2_2.len(), 400);
        let board2_3 = step_depth(&board2_2);
        assert_eq!(board2_3.len(), 8902);
        })
    }

    // Benches performance of move search.
    #[bench]
    fn test_movesearch(b: &mut Bencher) {
        let new = Board::from_fen("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1").unwrap();
        b.iter(|| {
            let _new_eval = crate::engine::alphabeta(
                &new,
                4,
                Score(-30001), 
                Score(30001),
                Color::not(new.meta.player));
        })
    }

    // Benches performance of process_move.
    #[bench]
    fn test_processmove(b: &mut Bencher) {

        b.iter(|| {
            let mut new = Board::from_fen("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1").unwrap();
            new.process_move(&Move {
                color: Color::White,
                piece: Piece::Pawn,
                origin: Square::E2,
                destination: Square::E4,
                promote_type: None,
                is_castle: false
            }).expect("oh no.");    
        })
    }

}

