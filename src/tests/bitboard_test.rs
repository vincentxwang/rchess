#[cfg(test)]
mod tests {

    use crate::core::constants::KNIGHT_START;
    use crate::game::bitboard::Bitboard as Bitboard;
    use crate::core::structs::Square as Square;

    // Bitboard tests
    #[test]
    fn test_insert() {
        let mut test: Bitboard = Bitboard::empty();
        test.insert(&Square::from_int(0));
        assert_eq!(Bitboard::to_integer(&test), 0b_00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000001);
        test.insert(&Square::from_int(1));
        test.insert(&Square::from_int(2));
        assert_eq!(Bitboard::to_integer(&test), 0b_00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000111);
    }

    #[test]
    fn test_is_piece() {
        let test1: Bitboard = Bitboard::empty();
        assert_eq!(false, test1.is_piece(&Square::A2));
        assert_eq!(false, test1.is_piece(&Square::G6));
        let test2: Bitboard = Bitboard::new(KNIGHT_START);
        assert_eq!(true, test2.is_piece(&Square::B1));
        assert_eq!(false, test2.is_piece(&Square::A2));
    }
    
}