#[derive(Copy, Clone, Debug)]
pub enum Color {
    White = 0,
    Black = 1,
}

impl PartialEq for Color {
    fn eq(&self, other: &Self) -> bool {
        *self as usize == *other as usize
    }
}


#[derive(Copy, Clone, Debug)]
pub enum Square {
    A1 = 0,
    B1 = 1,
    C1 = 2,
    D1 = 3,
    E1 = 4,
    F1 = 5,
    G1 = 6,
    H1 = 7,
    A2 = 8,
    B2 = 9,
    C2 = 10,
    D2 = 11,
    E2 = 12,
    F2 = 13,
    G2 = 14,
    H2 = 15,
    A3 = 16,
    B3 = 17,
    C3 = 18,
    D3 = 19,
    E3 = 20,
    F3 = 21,
    G3 = 22,
    H3 = 23,
    A4 = 24,
    B4 = 25,
    C4 = 26,
    D4 = 27,
    E4 = 28,
    F4 = 29,
    G4 = 30,
    H4 = 31,
    A5 = 32,
    B5 = 33,
    C5 = 34,
    D5 = 35,
    E5 = 36,
    F5 = 37,
    G5 = 38,
    H5 = 39,
    A6 = 40,
    B6 = 41,
    C6 = 42,
    D6 = 43,
    E6 = 44,
    F6 = 45,
    G6 = 46,
    H6 = 47,
    A7 = 48,
    B7 = 49,
    C7 = 50,
    D7 = 51,
    E7 = 52,
    F7 = 53,
    G7 = 54,
    H7 = 55,
    A8 = 56,
    B8 = 57,
    C8 = 58,
    D8 = 59,
    E8 = 60,
    F8 = 61,
    G8 = 62,
    H8 = 63,
}

impl Square {
    // Converts integer to Square.
    pub fn from_int(num: usize) -> Square {
        match num {
            0 => Square::A1,  1 => Square::B1,  2 => Square::C1,  3 => Square::D1,
            4 => Square::E1,  5 => Square::F1,  6 => Square::G1,  7 => Square::H1,
            8 => Square::A2,  9 => Square::B2, 10 => Square::C2, 11 => Square::D2,
            12 => Square::E2, 13 => Square::F2, 14 => Square::G2, 15 => Square::H2,
            16 => Square::A3, 17 => Square::B3, 18 => Square::C3, 19 => Square::D3,
            20 => Square::E3, 21 => Square::F3, 22 => Square::G3, 23 => Square::H3,
            24 => Square::A4, 25 => Square::B4, 26 => Square::C4, 27 => Square::D4,
            28 => Square::E4, 29 => Square::F4, 30 => Square::G4, 31 => Square::H4,
            32 => Square::A5, 33 => Square::B5, 34 => Square::C5, 35 => Square::D5,
            36 => Square::E5, 37 => Square::F5, 38 => Square::G5, 39 => Square::H5,
            40 => Square::A6, 41 => Square::B6, 42 => Square::C6, 43 => Square::D6,
            44 => Square::E6, 45 => Square::F6, 46 => Square::G6, 47 => Square::H6,
            48 => Square::A7, 49 => Square::B7, 50 => Square::C7, 51 => Square::D7,
            52 => Square::E7, 53 => Square::F7, 54 => Square::G7, 55 => Square::H7,
            56 => Square::A8, 57 => Square::B8, 58 => Square::C8, 59 => Square::D8,
            60 => Square::E8, 61 => Square::F8, 62 => Square::G8, 63 => Square::H8,
            _ => panic!("tried to convert some invalid value: {} to Square", num),
        }
    }

    // Converts string representation to square.
    pub fn from_str(str: String) -> Square {
        let mut str_chars = str.chars();
        let col = match str_chars.next().ok_or("incomplete file while converting string to square").unwrap().to_ascii_uppercase() {
            'A' => 0,
            'B' => 1,
            'C' => 2,
            'D' => 3,
            'E' => 4,
            'F' => 5,
            'G' => 6,
            'H' => 7,
            _ => panic!("invalid column while converting string to square")
        };
        let row = str_chars
            .next()
            .ok_or("incomplete rank while converting string to square")
            .unwrap()
            .to_digit(10)
            .unwrap();
        // We subtract one from row because rows in chess start counting from 1.
        Square::from_int((row as usize - 1) * 8 + col)
    }

    // Returns an integer that represents the file (vertical).
    pub fn get_file(&self) -> usize {
        *self as usize % 8
    }

    // Returns an integer that represents the rank (horizontal).
    pub fn get_rank(&self) -> usize {
        *self as usize / 8 + 1
    }
    // Gets the Manhattan metric between two squares!
    pub fn distance(sq1: Square, sq2: Square) -> usize {
        ((sq1.get_rank() as i32 - sq2.get_rank() as i32).abs() + (sq2.get_file() as i32 - sq1.get_file() as i32).abs()) as usize
    }

    // Gets square that is rank away and file away from another square.
    // Note that sign_x, sign_y take on false if the respective coordinate is supposed to be negative, and true if the respective coordinate 
    // is supposed to be positive. I made it this way because I couldn't add an isize to a usize.
    pub fn from_distance(sq: &Square, rank_x: usize, file_y: usize, sign_x: bool, sign_y: bool) -> Square {
        let mut num = *sq as usize;
        if sign_x {
            num += rank_x * 8;
        } else {
            num -= rank_x * 8;
        }

        if sign_y {
            num += file_y;
        } else {
            num -= file_y;
        }

        Square::from_int(num)
    }
    
}

impl PartialEq for Square {
    fn eq(&self, other: &Self) -> bool {
        *self as usize == *other as usize
    }
}