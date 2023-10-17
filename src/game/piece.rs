#[derive(Copy, Clone, Debug)]
pub enum Piece {
    Pawn = 0,
    Knight = 1,
    Bishop = 2,
    Rook = 3,
    Queen = 4,
    King = 5,
}

impl Piece {
    // Converts piece to string (i.e. gets code of Piece).
    pub const fn code(piece: Piece) -> char {
        match piece {
            Piece::Pawn => 'P',
            Piece::Knight => 'N',
            Piece::Bishop => 'B',
            Piece::Rook => 'R',
            Piece::Queen => 'Q',
            Piece::King => 'K',
        }
    }

    // Gets piece from piece string code.
    pub const fn from_code(chr: char) -> Piece {
        match chr {
            'P' => Piece::Pawn,
            'N' => Piece::Knight,
            'B' => Piece::Bishop,
            'R' => Piece::Rook,
            'Q' => Piece::Queen,
            'K' => Piece::King,
            _ => panic!("invalid character when converting from character to Piece!")
        }
    }

    // Gets piece from piece usize id.
    pub const fn from_id(id: usize) -> Piece {
        match id {
            0 => Piece::Pawn,
            1 => Piece::Knight,
            2 => Piece::Bishop,
            3 => Piece::Rook,
            4 => Piece::Queen,
            5 => Piece::King,
            _ => panic!("invalid piece id! piece_Id")
        }
    }
}

impl PartialEq for Piece {
    fn eq(&self, other: &Self) -> bool {
        *self as usize == *other as usize
    }
}