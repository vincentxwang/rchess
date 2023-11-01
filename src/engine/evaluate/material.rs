use crate::game::piece::Piece;
use crate::engine::evaluate::Score;


// All piece values are in "centipawns."
pub fn piece_eval(piece: Piece) -> Score {
    match piece {
        Piece::Pawn => Score(100),
        Piece::Knight => Score(320),
        Piece::Bishop => Score(330),
        Piece::Rook => Score(500),
        Piece::Queen => Score(900),
        Piece::King => Score(0),
    }
}