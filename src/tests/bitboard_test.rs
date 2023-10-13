#[cfg(test)]
mod tests {

    use crate::core::*;
    use crate::core::structs::Color as Color;
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
    
}