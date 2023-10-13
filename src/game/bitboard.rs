use crate::core::structs::Square as Square;

#[derive(Copy, Clone, Debug)]
// This is a tuple struct! We can access u64 through Bitboard.0
pub struct Bitboard(u64);

impl Bitboard {

    // Creates a bitboard equivalent to an integer.
    pub const fn new(x: u64) -> Bitboard {
        Bitboard(x)
    }
    
    pub const fn empty() -> Bitboard {
        Bitboard::new(0)
    }

    pub fn to_integer(&self) -> u64 {
        self.0
    }

    pub fn insert(&mut self, square: &Square) {
        self.0 |= 1 << *square as u8;
        println!("inserting {:?} to make {}", *square as u8, self.0)
    }
}