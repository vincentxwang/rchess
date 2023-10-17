use crate::game::piece::Piece as Piece; 
use crate::core::structs::Square as Square;
use crate::core::structs::Color as Color;
use crate::game::board::Board as Board;

// Move represents a single move from one side on a chessboard. This is otherwise called a "half-move."
pub struct Move {
    pub color: Color,
    pub piece: Piece,
    pub origin: Square,
    pub destination: Square,
    pub promote_type: Option<Piece>,
    pub is_castle: bool,
}

impl Move {

    // Converts a UCI move (long algebraic notation) to a Move. This will assume the UCI move being put is legal, given the origin piece exists.
    // !!!!!!!!! is_check is NOT implemented.  
    pub fn from_uci(board: &Board, str: &str) -> Move {
        let mut str_chrs = str.chars();

        let origin_file = str_chrs.next().unwrap();
        let origin_rank = str_chrs.next().unwrap();
        let origin_square = Square::from_str(origin_file.to_string() + &origin_rank.to_string());

        let destination_file = str_chrs.next().unwrap();
        let destination_rank = str_chrs.next().unwrap();
        let destination_square = Square::from_str(destination_file.to_string() + &destination_rank.to_string());

        let promotion = {
            let next = str_chrs.next();
            if next.is_some() {
                Some(Piece::from_code(next.unwrap().to_ascii_uppercase()))
            } else {
                None
            }
        };

        let color = {
            if board.sides[0].is_piece(&origin_square) {
                Color::White
            } else if board.sides[1].is_piece(&origin_square) {
                Color::Black
            } else {
                panic!("uci move has no piece to move on origin square!")
            }
        };

        let piece = {
            let mut piece_id = 0;
            while piece_id < 6 {
                if board.pieces[piece_id].is_piece(&origin_square) {
                    break;
                }
                piece_id += 1;
            }
            if piece_id >= 6 {
                panic!("something really really bad happened here. there is a piece that is findable in 
                white/black bitboards but not in the piece bitboards.")
            }
            Piece::from_id(piece_id)
        };

        // if piece is king

        let mut is_castle = false;

        if piece == Piece::King && Square::distance(origin_square, destination_square) > 1 {
            is_castle = true;
        }

        Move {
            color: color,
            piece: piece,
            origin: origin_square,
            destination: destination_square,
            promote_type: promotion,
            is_castle: is_castle,
        }
    }
    
    // generate a vector of all possible knight moves from a single square. a move is just anything that is possible on the board. we would still need to check
    // (i) legality, (ii) destination square is not moved on.
    pub fn generate_all_knight_moves(origin: &Square, color: &Color) -> Vec<Move> {
        let mut knight_squares: Vec<Square> = Vec::new();

        let file = origin.get_file();
        let rank = origin.get_rank();

        if file >= 2 && rank <= 6 {
            knight_squares.push(Square::from_distance(&origin, 2, 1, false, true));
        } 

        if file >= 2 && rank >= 1 {
            knight_squares.push(Square::from_distance(&origin, 2, 1, false, false));
        }

        if file >= 1 && rank >= 2 {
            knight_squares.push(Square::from_distance(&origin, 1, 2, false, false));
        }

        if file <= 6 && rank >= 2 {
            knight_squares.push(Square::from_distance(&origin, 1, 2, true, false));
        }

        if file <= 5 && rank >= 1 {
            knight_squares.push(Square::from_distance(&origin, 2, 1, true, false));
        }

        if file <= 5 && rank <= 6 {
            knight_squares.push(Square::from_distance(&origin, 2, 1, true, true));
        }

        if file <= 6 && rank <= 5 {
            knight_squares.push(Square::from_distance(&origin, 1, 2, true, true));
        }

        if file >= 1 && rank <= 5 {
            knight_squares.push(Square::from_distance(&origin, 1, 2, false, true));
        }

        let mut knight_moves = Vec::new();

        while knight_squares.len() != 0 {
            knight_moves.push( Move {
                color: *color,
                origin: *origin,
                piece: Piece::Knight,
                destination: knight_squares.pop().unwrap(),
                promote_type: None,
                is_castle: false,
            }
            );
        }
        knight_moves
    }

    /*
    pub fn generate_all_bishop_moves(board: &Board, origin: &Square, color: &Color) -> Vec<Move> {
        
    }
    */


}