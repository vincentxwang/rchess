#[cfg(test)]
mod tests {

    use crate::core::structs::Color as Color;
    use crate::core::structs::Square as Square;
    use crate::game::board::Board;
    use crate::game::piece::Piece as Piece;

    
    #[test]
    fn test_add_piece() {
        let mut game = Board::empty();
        game.add_piece(&Color::White, &Piece::Knight, &Square::H4);
        assert_eq!(game.pieces[1].to_integer(), 1 << 31);
    }

    /* this should probably be done at some point...
    #[test]
    fn test_from_fen() {

    }
    */
}