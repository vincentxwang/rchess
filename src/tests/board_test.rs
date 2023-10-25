#[cfg(test)]
mod tests {

    use crate::core::structs::Color as Color;
    use crate::core::structs::Square as Square;
    use crate::game::board::Board;
    use crate::game::piece::Piece as Piece;
    use crate::core::structs::Direction as Direction;

    
    #[test]
    fn test_add_piece() {
        let mut game = Board::empty();
        game.add_piece(&Color::White, &Piece::Knight, &Square::H4);
        assert_eq!(game.pieces[1].to_integer(), 1 << 31);
    }

    #[test]
    fn test_get_furthest_piece_along_ray() {
        let test1 = Board::from_fen("rnbqkbnr/pppppppp/8/8/4P3/8/PPPP1PPP/RNBQKBNR w KQkq e3 0 0").unwrap();
        assert_eq!(test1.get_furthest_piece_along_ray(&Square::D3, Direction::North), Some((Piece::Pawn, Color::Black)));
        // Note that get_furthest_piece_along_ray should return "None" when encountering a piece of the same color (i.e. white).
        assert_eq!(test1.get_furthest_piece_along_ray(&Square::D6, Direction::South), None);
        assert_eq!(test1.get_furthest_piece_along_ray(&Square::D6, Direction::East), None);
    }

    #[test]
    fn test_is_attacked() {
        let test1 = Board::from_fen("r3kbnr/1bp2ppp/p1n1p3/qp4B1/3P4/1BN2N2/PPP2PPP/R2QK2R b KQkq - 0 1").unwrap();
        assert_eq!(test1.is_attacked(&Square::D8), true);
        assert_eq!(test1.is_attacked(&Square::E8), false);
        assert_eq!(test1.is_attacked(&Square::D7), false);
        assert_eq!(test1.is_attacked(&Square::G8), false);
        assert_eq!(test1.is_attacked(&Square::F7), false);
        assert_eq!(test1.is_attacked(&Square::E6), true);
        let test2 = Board::from_fen("8/3k4/4pbn1/p2Pp1BP/pP1P4/2N3q1/K7/6R1 w - - 0 1").unwrap();
        assert_eq!(test2.is_attacked(&Square::A2), false);
        assert_eq!(test2.is_attacked(&Square::A8), false);
        assert_eq!(test2.is_attacked(&Square::G1), true);
        assert_eq!(test2.is_attacked(&Square::D4), true);
        assert_eq!(test2.is_attacked(&Square::C3), true);
    }
    
    /* this should probably be done at some point...
    #[test]
    fn test_from_fen() {

    }
    */
}