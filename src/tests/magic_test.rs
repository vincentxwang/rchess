#[cfg(test)]
mod tests {
    use crate::game::magic::*;
    use crate::core::structs::Square as Square;
    
    #[test]
    fn test_north_ray_attacks() {
        assert_eq!(NORTH_RAY_ATTACKS[Square::B1 as usize].to_integer(), 0b_00000010_00000010_00000010_00000010_00000010_00000010_00000010_00000000);
        assert_eq!(NORTH_RAY_ATTACKS[Square::A2 as usize].to_integer(), 0b_00000001_00000001_00000001_00000001_00000001_00000001_00000000_00000000);
        assert_eq!(NORTH_RAY_ATTACKS[Square::C4 as usize].to_integer(), 0b_00000100_00000100_00000100_00000100_00000000_00000000_00000000_00000000);
        assert_eq!(NORTH_RAY_ATTACKS[Square::H8 as usize].to_integer(), 0);
    }

    #[test]
    fn test_south_ray_attacks() {
        assert_eq!(SOUTH_RAY_ATTACKS[Square::B1 as usize].to_integer(), 0);
        assert_eq!(SOUTH_RAY_ATTACKS[Square::A2 as usize].to_integer(), 0b_00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000001);
        assert_eq!(SOUTH_RAY_ATTACKS[Square::C4 as usize].to_integer(), 0b_00000000_00000000_00000000_00000000_00000000_00000100_00000100_00000100);
        assert_eq!(SOUTH_RAY_ATTACKS[Square::H8 as usize].to_integer(), 0b_00000000_10000000_10000000_10000000_10000000_10000000_10000000_10000000);
    }

    #[test]
    fn test_northeast_ray_attacks() {
        assert_eq!(NORTHEAST_RAY_ATTACKS[Square::B1 as usize].to_integer(), 0b_00000000_10000000_01000000_00100000_00010000_00001000_00000100_00000000);
        assert_eq!(NORTHEAST_RAY_ATTACKS[Square::A2 as usize].to_integer(), 0b_01000000_00100000_00010000_00001000_00000100_00000010_00000000_00000000);
        assert_eq!(NORTHEAST_RAY_ATTACKS[Square::C4 as usize].to_integer(), 0b_01000000_00100000_00010000_00001000_00000000_00000000_00000000_00000000);
        assert_eq!(NORTHEAST_RAY_ATTACKS[Square::H8 as usize].to_integer(), 0b_00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000);
    }

    #[test]
    fn test_northwest_ray_attacks() {
        assert_eq!(NORTHWEST_RAY_ATTACKS[Square::B1 as usize].to_integer(), 0b_00000000_00000000_00000000_00000000_00000000_00000000_00000001_00000000);
        assert_eq!(NORTHWEST_RAY_ATTACKS[Square::A2 as usize].to_integer(), 0);
        assert_eq!(NORTHWEST_RAY_ATTACKS[Square::C4 as usize].to_integer(), 0b_00000000_00000000_00000001_00000010_00000000_00000000_00000000_00000000);
        assert_eq!(NORTHWEST_RAY_ATTACKS[Square::H8 as usize].to_integer(), 0);
    }

    #[test]
    fn test_west_ray_attacks() {
        assert_eq!(WEST_RAY_ATTACKS[Square::B1 as usize].to_integer(), 0b_00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000001);
        assert_eq!(WEST_RAY_ATTACKS[Square::A2 as usize].to_integer(), 0);
        assert_eq!(WEST_RAY_ATTACKS[Square::C4 as usize].to_integer(), 0b_00000000_00000000_00000000_00000000_00000011_00000000_00000000_00000000);
        assert_eq!(WEST_RAY_ATTACKS[Square::H8 as usize].to_integer(), 0b_01111111_00000000_00000000_00000000_00000000_00000000_00000000_00000000);
    }


    #[test]
    fn test_east_ray_attacks() {
        assert_eq!(EAST_RAY_ATTACKS[Square::B1 as usize].to_integer(), 0b_00000000_00000000_00000000_00000000_00000000_00000000_00000000_11111100);
        assert_eq!(EAST_RAY_ATTACKS[Square::A2 as usize].to_integer(), 0b_00000000_00000000_00000000_00000000_00000000_00000000_11111110_00000000);
        assert_eq!(EAST_RAY_ATTACKS[Square::C4 as usize].to_integer(), 0b_00000000_00000000_00000000_00000000_11111000_00000000_00000000_00000000);
        assert_eq!(EAST_RAY_ATTACKS[Square::H8 as usize].to_integer(), 0);
    }

    #[test]
    fn test_southwest_ray_attacks() {
        assert_eq!(SOUTHWEST_RAY_ATTACKS[Square::B1 as usize].to_integer(), 0);
        assert_eq!(SOUTHWEST_RAY_ATTACKS[Square::A2 as usize].to_integer(), 0);
        assert_eq!(SOUTHWEST_RAY_ATTACKS[Square::C4 as usize].to_integer(), 0b_00000000_00000000_00000000_00000000_00000000_00000010_00000001_00000000);
        assert_eq!(SOUTHWEST_RAY_ATTACKS[Square::H7 as usize].to_integer(), 0b_00000000_00000000_01000000_00100000_00010000_00001000_00000100_00000010);
    }

    #[test]
    fn test_southeast_ray_attacks() {
        assert_eq!(SOUTHEAST_RAY_ATTACKS[Square::B1 as usize].to_integer(), 0);
        assert_eq!(SOUTHEAST_RAY_ATTACKS[Square::A2 as usize].to_integer(), 0b_00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000010);
        assert_eq!(SOUTHEAST_RAY_ATTACKS[Square::C4 as usize].to_integer(), 0b_00000000_00000000_00000000_00000000_00000000_00001000_00010000_00100000);
        assert_eq!(SOUTHEAST_RAY_ATTACKS[Square::H7 as usize].to_integer(), 0);
    }

}