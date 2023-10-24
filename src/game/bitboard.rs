use crate::core::structs::Square as Square;

#[derive(Copy, Clone, Debug)]
// This is a tuple struct! We can access u64 through Bitboard.0
pub struct Bitboard(u64);

impl Bitboard {

    // Creates a bitboard equivalent to an integer.
    pub const fn new(x: u64) -> Bitboard {
        Bitboard(x)
    }
    
    // Constructs an empty bitboard.
    pub const fn empty() -> Bitboard {
        Bitboard::new(0)
    }

    // Converts a bitboard to a u64 integer.
    pub fn to_integer(&self) -> u64 {
        self.0
    }

    // Insert a bit into a bitboard.
    pub fn insert(&mut self, square: &Square) {
        self.0 |= 1 << *square as u8;
        println!("inserting {:?} to make {}", *square as u8, self.0)
    }

    // Shifts bitboard left by an integer.
    pub fn shift_left(&mut self, x: usize) {
        self.0 <<= x;
    }

    // Shifts bitboard right by an integer.
    pub fn shift_right(&mut self, x: usize) {
        self.0 >>= x;
    }

    // Determines if there is a 1-bit at a given square.
    pub fn is_piece(&self, square: &Square) -> bool {
        self.0 & 1 << *square as u8 != 0
    }

    // Sets a bit to zero.
    pub fn set_zero(&mut self, square: &Square) {
        self.0 &= !(1 << *square as u8);
    }

    // Toggles a bit.
    pub fn toggle(&mut self, square: &Square) {
        self.0 ^= 1 << *square as u8
    }

    // Toggles most significant 1 to a 0.
    pub fn toggle_msb(&mut self) {
        if self.0.leading_zeros() == 64 {
            panic!("tried to toggle most significant bit, but bitboard is empty!")
        }
        self.toggle(&Square::from_int(63 - self.0.leading_zeros() as usize))
    }

    // Toggles most significant 1 to a 0.
    pub fn toggle_lsb(&mut self) {
        if self.0.leading_zeros() == 64 {
            panic!("tried to toggle least significant bit, but bitboard is empty!")
        }
        self.toggle(&Square::from_int(self.0.trailing_zeros() as usize))       
    }
    
    // Scans for the least significant bit on "intersection" bitboard and removes all bits after the least significant bit on self.
    pub fn bitscan_forward(&mut self, intersection: &Bitboard) {
        if intersection.0.leading_zeros() != 64 {
            let lsb = intersection.0.trailing_zeros() as usize;
            for i in lsb..64 {

            }
        }
    }


}