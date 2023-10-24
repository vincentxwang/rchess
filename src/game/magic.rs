// magic.rs provides "magic" bitboards that are pregenerated to generate moves.
use crate::game::bitboard::Bitboard as Bitboard;

// Generates an array of Bitboards. Each bitboard represents a ray attack from a square. 
// A ray attack is all possible squares to move on an empty board in a certain direction.
lazy_static! {
    pub static ref NORTH_RAY_ATTACKS: [Bitboard; 64] = compute_north_ray_attacks();
    pub static ref SOUTH_RAY_ATTACKS: [Bitboard; 64] = compute_south_ray_attacks();
    pub static ref NORTHEAST_RAY_ATTACKS: [Bitboard; 64] = compute_northeast_ray_attacks();
    pub static ref NORTHWEST_RAY_ATTACKS: [Bitboard; 64] = compute_northwest_ray_attacks();
    pub static ref WEST_RAY_ATTACKS: [Bitboard; 64] = compute_west_ray_attacks();
    pub static ref EAST_RAY_ATTACKS: [Bitboard; 64] = compute_east_ray_attacks();
    pub static ref SOUTHWEST_RAY_ATTACKS: [Bitboard; 64] = compute_southwest_ray_attacks();
    pub static ref SOUTHEAST_RAY_ATTACKS: [Bitboard; 64] = compute_southeast_ray_attacks();
}

fn compute_north_ray_attacks() -> [Bitboard; 64] {
    // NORTH_RAY_ATTACKS at a1, then we shift 1 left to get all the others.
    let mut nort: u64 = 0b_00000001_00000001_00000001_00000001_00000001_00000001_00000001_00000000;
    let mut attacks = [Bitboard::empty(); 64];
    for sq in 0..64 {
        attacks[sq] = Bitboard::new(nort);
        nort <<= 1;
    };
    attacks
}

fn compute_south_ray_attacks() -> [Bitboard; 64]{
    // SOUTH_RAY_ATTACKS at h8, then we shift 1 right to get all others.
    let mut sout: u64 = 0b_00000000_10000000_10000000_10000000_10000000_10000000_10000000_10000000;
    let mut attacks = [Bitboard::empty(); 64];
    for sq in (0..64).rev() {
        attacks[sq] = Bitboard::new(sout);
        sout >>= 1;
    }
    attacks
}

fn compute_northeast_ray_attacks() -> [Bitboard; 64] {
    // NORTHEAST_RAY_ATTACKS at a1.
    let mut northeast = Bitboard::new(0b_10000000_01000000_00100000_00010000_00001000_00000100_00000010_00000000);
    let mut attacks = [Bitboard::empty(); 64];
    for file in 0..8 {
        let mut temp_northeast = northeast;
        for rank in 0..8 {
            attacks[rank * 8 + file] = temp_northeast;
            temp_northeast.shift_left(8);
        }
        if file == 7 {
            break;
        }
        northeast.toggle_msb();
        northeast.shift_left(1);
    }
    attacks
}

fn compute_northwest_ray_attacks() -> [Bitboard; 64] {
    // NORTHWEST_RAY_ATTACKS at h1.
    let mut northwest = Bitboard::new(0b_00000001_00000010_00000100_00001000_00010000_00100000_01000000_00000000);
    let mut attacks = [Bitboard::empty(); 64];
    for file in (0..8).rev() {
        let mut temp_northwest = northwest;
        for rank in 0..8 {
            attacks[rank * 8 + file] = temp_northwest;
            temp_northwest.shift_left(8);
        }
        // We break on the last one -- otherwise it would try to remove a bit that doesn't exist.
        if file == 0 {
            break;
        }
        northwest.toggle_msb();
        northwest.shift_right(1);
    }
    attacks
}

fn compute_west_ray_attacks() -> [Bitboard; 64] {
    // WEST_RAY_ATTACKS at h1.
    let mut west = Bitboard::new(0b_00000000_00000000_00000000_00000000_00000000_00000000_00000000_01111111);
    let mut attacks = [Bitboard::empty(); 64];
    for file in (0..8).rev() {
        let mut temp_west = west;
        for rank in 0..8 {
            attacks[rank * 8 + file] = temp_west;
            temp_west.shift_left(8);
        }
        west.shift_right(1);
    }
    attacks
}

fn compute_east_ray_attacks() -> [Bitboard; 64] {
    // EAST_RAY_ATTACKS at a1.
    let mut east = Bitboard::new(0b_00000000_00000000_00000000_00000000_00000000_00000000_00000000_11111110);
    let mut attacks = [Bitboard::empty(); 64];
    for file in 0..8 {
        let mut temp_east = east;
        for rank in 0..8 {
            attacks[rank * 8 + file] = temp_east;
            temp_east.shift_left(8);
        }
        // We break on the last one -- otherwise it would try to remove a bit that doesn't exist.
        if file == 7 {
            break;
        }
        east.toggle_msb();
        east.shift_left(1);
    }
    attacks
}

fn compute_southwest_ray_attacks() -> [Bitboard; 64] {
    // SOUTHWEST_RAY_ATTACKS at h8.
    let mut southwest = Bitboard::new(0b_00000000_01000000_00100000_00010000_00001000_00000100_00000010_00000001);
    let mut attacks = [Bitboard::empty(); 64];
    for file in (0..8).rev() {
        let mut temp_southwest = southwest;
        for rank in (0..8).rev() {
            attacks[rank * 8 + file] = temp_southwest;
            temp_southwest.shift_right(8);
        }
        southwest.shift_right(1);
    }
    attacks
}

fn compute_southeast_ray_attacks() -> [Bitboard; 64] {
    // SOUTHEAST_RAY_ATTACKS at a8.
    let mut southeast = Bitboard::new(0b_00000000_00000010_00000100_00001000_00010000_00100000_01000000_10000000);
    let mut attacks = [Bitboard::empty(); 64];
    for file in 0..8 {
        let mut temp_southeast = southeast;
        for rank in (0..8).rev() {
            attacks[rank * 8 + file] = temp_southeast;
            temp_southeast.shift_right(8);
        }
        // We break on the last one -- otherwise it would try to remove a bit that doesn't exist.
        if file == 7 {
            break;
        }
        southeast.toggle_lsb();
        southeast.shift_left(1);
    }
    attacks
}