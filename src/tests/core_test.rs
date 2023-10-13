#[cfg(test)]
mod tests {

    use crate::core::*;
    use crate::core::structs::Color as Color;
    use crate::core::structs::Square as Square;

    // Color tests
    #[test]
    fn test_opposite_colors() {
        assert_ne!(Color::White as u64, Color::Black as u64);
    }

    // Square tests
    #[test]
    fn test_square_integer() {
        assert_eq!(Square::A2 as usize, Square::from_int(8) as usize);
        assert_eq!(Square::B1 as usize, Square::from_int(1) as usize);
    }

    #[test]
    fn test_from_str() {
        assert_eq!(Square::A2 as usize, Square::from_str("a2".to_string()) as usize);
        assert_eq!(Square::H2 as usize, Square::from_str("h2".to_string()) as usize);
        assert_eq!(Square::G7 as usize, Square::from_str("g7".to_string()) as usize);
        assert_eq!(Square::G8 as usize, Square::from_str("g8".to_string()) as usize);
    }

    /*
    fn test_get_file() {
        assert_eq!(Square::A2.get_file(), 'A' as u8);
        assert_eq!(Square::D3.get_file(), 'D' as u8);
        assert_eq!(Square::H8.get_file(), 'H' as u8);
    }
    */

    /*
    fn test_square_define() {
        assert_eq!(Square(0), Square::A2);
    }
    */
    
}