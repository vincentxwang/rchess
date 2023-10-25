#[cfg(test)]
mod tests {
    use crate::game::magic::*;
    use crate::core::structs::Square as Square;
    use crate::game::board::Board as Board;
    use crate::core::structs::Direction as Direction;
    use crate::game::moves::Move as Move;
    
    #[test]
    fn test_north_ray_attacks() {
        assert_eq!(RAY_ATTACKS[Direction::North as usize][Square::B1 as usize].to_integer(), 0b_00000010_00000010_00000010_00000010_00000010_00000010_00000010_00000000);
        assert_eq!(RAY_ATTACKS[Direction::North as usize][Square::A2 as usize].to_integer(), 0b_00000001_00000001_00000001_00000001_00000001_00000001_00000000_00000000);
        assert_eq!(RAY_ATTACKS[Direction::North as usize][Square::C4 as usize].to_integer(), 0b_00000100_00000100_00000100_00000100_00000000_00000000_00000000_00000000);
        assert_eq!(RAY_ATTACKS[Direction::North as usize][Square::H8 as usize].to_integer(), 0);
    }

    #[test]
    fn test_south_ray_attacks() {
        assert_eq!(RAY_ATTACKS[Direction::South as usize][Square::B1 as usize].to_integer(), 0);
        assert_eq!(RAY_ATTACKS[Direction::South as usize][Square::A2 as usize].to_integer(), 0b_00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000001);
        assert_eq!(RAY_ATTACKS[Direction::South as usize][Square::C4 as usize].to_integer(), 0b_00000000_00000000_00000000_00000000_00000000_00000100_00000100_00000100);
        assert_eq!(RAY_ATTACKS[Direction::South as usize][Square::H8 as usize].to_integer(), 0b_00000000_10000000_10000000_10000000_10000000_10000000_10000000_10000000);
    }

    #[test]
    fn test_northeast_ray_attacks() {
        assert_eq!(RAY_ATTACKS[Direction::Northeast as usize][Square::B1 as usize].to_integer(), 0b_00000000_10000000_01000000_00100000_00010000_00001000_00000100_00000000);
        assert_eq!(RAY_ATTACKS[Direction::Northeast as usize][Square::A2 as usize].to_integer(), 0b_01000000_00100000_00010000_00001000_00000100_00000010_00000000_00000000);
        assert_eq!(RAY_ATTACKS[Direction::Northeast as usize][Square::C4 as usize].to_integer(), 0b_01000000_00100000_00010000_00001000_00000000_00000000_00000000_00000000);
        assert_eq!(RAY_ATTACKS[Direction::Northeast as usize][Square::H8 as usize].to_integer(), 0b_00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000);
    }

    #[test]
    fn test_northwest_ray_attacks() {
        assert_eq!(RAY_ATTACKS[Direction::Northwest as usize][Square::B1 as usize].to_integer(), 0b_00000000_00000000_00000000_00000000_00000000_00000000_00000001_00000000);
        assert_eq!(RAY_ATTACKS[Direction::Northwest as usize][Square::A2 as usize].to_integer(), 0);
        assert_eq!(RAY_ATTACKS[Direction::Northwest as usize][Square::C4 as usize].to_integer(), 0b_00000000_00000000_00000001_00000010_00000000_00000000_00000000_00000000);
        assert_eq!(RAY_ATTACKS[Direction::Northwest as usize][Square::H8 as usize].to_integer(), 0);
    }

    #[test]
    fn test_west_ray_attacks() {
        assert_eq!(RAY_ATTACKS[Direction::West as usize][Square::B1 as usize].to_integer(), 0b_00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000001);
        assert_eq!(RAY_ATTACKS[Direction::West as usize][Square::A2 as usize].to_integer(), 0);
        assert_eq!(RAY_ATTACKS[Direction::West as usize][Square::C4 as usize].to_integer(), 0b_00000000_00000000_00000000_00000000_00000011_00000000_00000000_00000000);
        assert_eq!(RAY_ATTACKS[Direction::West as usize][Square::H8 as usize].to_integer(), 0b_01111111_00000000_00000000_00000000_00000000_00000000_00000000_00000000);
    }


    #[test]
    fn test_east_ray_attacks() {
        assert_eq!(RAY_ATTACKS[Direction::East as usize][Square::B1 as usize].to_integer(), 0b_00000000_00000000_00000000_00000000_00000000_00000000_00000000_11111100);
        assert_eq!(RAY_ATTACKS[Direction::East as usize][Square::A2 as usize].to_integer(), 0b_00000000_00000000_00000000_00000000_00000000_00000000_11111110_00000000);
        assert_eq!(RAY_ATTACKS[Direction::East as usize][Square::C4 as usize].to_integer(), 0b_00000000_00000000_00000000_00000000_11111000_00000000_00000000_00000000);
        assert_eq!(RAY_ATTACKS[Direction::East as usize][Square::H8 as usize].to_integer(), 0);
    }

    #[test]
    fn test_southwest_ray_attacks() {
        assert_eq!(RAY_ATTACKS[Direction::Southwest as usize][Square::B1 as usize].to_integer(), 0);
        assert_eq!(RAY_ATTACKS[Direction::Southwest as usize][Square::A2 as usize].to_integer(), 0);
        assert_eq!(RAY_ATTACKS[Direction::Southwest as usize][Square::C4 as usize].to_integer(), 0b_00000000_00000000_00000000_00000000_00000000_00000010_00000001_00000000);
        assert_eq!(RAY_ATTACKS[Direction::Southwest as usize][Square::H7 as usize].to_integer(), 0b_00000000_00000000_01000000_00100000_00010000_00001000_00000100_00000010);
    }

    #[test]
    fn test_southeast_ray_attacks() {
        assert_eq!(RAY_ATTACKS[Direction::Southeast as usize][Square::B1 as usize].to_integer(), 0);
        assert_eq!(RAY_ATTACKS[Direction::Southeast as usize][Square::A2 as usize].to_integer(), 0b_00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000010);
        assert_eq!(RAY_ATTACKS[Direction::Southeast as usize][Square::C4 as usize].to_integer(), 0b_00000000_00000000_00000000_00000000_00000000_00001000_00010000_00100000);
        assert_eq!(RAY_ATTACKS[Direction::Southeast as usize][Square::H7 as usize].to_integer(), 0);
    }

    #[test]
    fn test_get_postive_rays_attacks() {
        let board1 = Board::from_fen("r1bqkbnr/p1pppppp/1pn5/8/8/6P1/PPPPPPBP/RNBQK1NR b KQkq e3 0 0").unwrap();
        board1.sides[0].print_bitboard();
        board1.sides[1].print_bitboard();
        assert_eq!(Move::get_positive_ray_attacks(&board1, &Square::G2, Direction::Northwest).to_integer(), 0b_00000000_00000000_00000100_00001000_00010000_00100000_00000000_00000000);
    }

}