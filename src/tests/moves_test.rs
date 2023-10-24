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

}