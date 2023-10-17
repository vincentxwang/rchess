#[cfg(test)]
mod tests {

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
        assert_eq!(Square::A2, Square::from_int(8));
        assert_eq!(Square::B1, Square::from_int(1));
    }

    #[test]
    fn test_from_str() {
        assert_eq!(Square::A2, Square::from_str("a2".to_string()));
        assert_eq!(Square::H2, Square::from_str("h2".to_string()));
        assert_eq!(Square::G7, Square::from_str("g7".to_string()));
        assert_eq!(Square::G8, Square::from_str("g8".to_string()));
    }

    #[test]
    fn test_get_file() {
        assert_eq!(Square::A2.get_file(), 0);
        assert_eq!(Square::D3.get_file(), 3);
        assert_eq!(Square::H8.get_file(), 7);
    }

    #[test]
    fn test_get_rank() {
        assert_eq!(Square::A2.get_rank(), 2);
        assert_eq!(Square::A7.get_rank(), 7);
        assert_eq!(Square::H8.get_rank(), 8);
    }

    #[test]
    fn test_distance() {
        assert_eq!(Square::distance(Square::A2, Square::F2), 5);
        assert_eq!(Square::distance(Square::A1, Square::H8), 14);
    }
    
}