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

    pub fn and(&mut self, bitboard: &Bitboard) -> Bitboard {
        self.0 &= bitboard.0;
        *self
    }

    pub fn or(&mut self, bitboard: &Bitboard) -> Bitboard {
        self.0 |= bitboard.0;
        *self
    }

    pub fn xor(&mut self, bitboard: &Bitboard) -> Bitboard {
        self.0 ^= bitboard.0;
        *self
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
    
    // find_lsb assumes that self is nonempty.
    pub fn find_lsb(&self) -> Square {
        Square::from_int(self.0.trailing_zeros() as usize)
    }

    pub fn find_msb(&self) -> Square {
        Square::from_int(63 - self.0.leading_zeros() as usize)
    }

    // Prints a nice 8x8 array of a bitboard. Used for debugging.
    pub fn print_bitboard(&self) {
        println!("--------- Printing Bitboard ----------");
        println!("Bitboard u64: {}", self.0);
        for rank in (0..8).rev() {
            for file in 0..8 {
                if self.is_piece(&Square::from_int(8 * rank + file)) {
                    print!("1 ");
                } else {
                    print!("0 ");
                }
            }
            println!("");
        }
        println!("----------- End of Print ------------");
    }

    // Returns a vector of squares that a Bitboard represents.
    pub fn get_squares(&self) -> Vec<Square> {
        let mut bitboard = self.clone();
        let mut squares = Vec::new();
        while bitboard.0 != 0 {
            squares.push(bitboard.find_lsb());
            bitboard.toggle(&bitboard.find_lsb());
        }
        squares
    }
}

impl PartialEq for Bitboard {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0 
    }
}

