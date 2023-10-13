use crate::game::piece::Piece as Piece; 
use crate::core::structs::Square as Square;
use crate::core::structs::Color as Color;

// Move represents a single move from one side on a chessboard. This is sometimes called a "half-move."
pub struct Move {
    color: Color,
    piece: Piece,
    origin: Square,
    destination: Square,
}

impl Move {

}