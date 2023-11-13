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
        assert!(!test1.is_piece(&Square::A2));
        assert!(!test1.is_piece(&Square::G6));
        let test2: Bitboard = Bitboard::new(KNIGHT_START);
        assert!(test2.is_piece(&Square::B1));
        assert!(!test2.is_piece(&Square::A2));
    }

    #[test]
    fn test_set_zero() {
        let mut test1 = Bitboard::new(0b_10000000_00000000_00000000_00000000_00000000_00000000_00000000_00000010);
        test1.set_zero(&Square::H8);
        assert_eq!(test1.to_integer(), 0b_00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000010)
    }

    #[test]
    fn test_toggle() {
        let mut test1 = Bitboard::empty();
        test1.toggle(&Square::B1);
        assert_eq!(test1.to_integer(), 0b_00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000010);
    }

    #[test]
    fn test_toggle_msb() {
        let mut test1 = Bitboard::new(0b_00000000_00000000_00010000_00000000_00000000_00000000_00000000_00000010);
        test1.toggle_msb();
        assert_eq!(test1.to_integer(), 2);
    }
    
    #[test]
    fn test_toggle_lsb() {
        let mut test1 = Bitboard::new(0b_00000000_00000000_00010000_00000000_00000000_00000000_00000000_00000010);
        test1.toggle_lsb();
        assert_eq!(test1.to_integer(), 0b_00000000_00000000_00010000_00000000_00000000_00000000_00000000_00000000);
    }
}