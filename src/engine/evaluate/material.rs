use crate::game::board::Board;
use crate::game::movegen::moves::Move;
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

pub fn mvv_lva(m: &Move, board: &Board) -> Score {
    if board.get_piece(&m.destination) != None {
        return piece_eval(board.get_piece(&m.destination).unwrap().0) - piece_eval(m.piece);
    } 
    Score(0)
}