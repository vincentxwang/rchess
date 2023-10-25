use crate::core::constants::PIECETYPE_COUNT;
use crate::core::structs::Direction;
use crate::game::piece::Piece as Piece; 
use crate::game::bitboard::Bitboard as Bitboard;
use crate::core::structs::Square as Square;
use crate::core::structs::Color as Color;
use crate::game::board::Board as Board;
use crate::game::magic::*;

// Move represents a single move from one side on a chessboard. This is otherwise called a "half-move."
#[derive(Debug)]
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
    pub fn generate_all_knight_moves(board: &Board, origin: &Square) -> Vec<Move> {
        let mover = board.meta.player;
        let mut knight_squares: Vec<Square> = Vec::new();

        let file = origin.get_file();
        let rank = origin.get_rank();

        // NWW
        if file >= 2 && rank <= 7 {
            knight_squares.push(Square::from_int(*origin as usize + 6));
        } 

        // SWW
        if file >= 2 && rank >= 2 {
            knight_squares.push(Square::from_int(*origin as usize - 10));
        }

        // SSW
        if file >= 1 && rank >= 3 {
            knight_squares.push(Square::from_int(*origin as usize - 17));
        }

        // SSE
        if file <= 6 && rank >= 3 {
            knight_squares.push(Square::from_int(*origin as usize - 15));
        }

        // SEE
        if file <= 5 && rank >= 2 {
            knight_squares.push(Square::from_int(*origin as usize - 6));
        }

        // NEE
        if file <= 5 && rank <= 7 {
            knight_squares.push(Square::from_int(*origin as usize + 10));
        }

        // NNE
        if file <= 6 && rank <= 6 {
            knight_squares.push(Square::from_int(*origin as usize + 17));
        }

        // NNW
        if file >= 1 && rank <= 6 {
            knight_squares.push(Square::from_int(*origin as usize + 15));
        }

        let mut knight_moves = Vec::new();

        while knight_squares.len() != 0 {
            let knight_square = knight_squares.pop().unwrap();
            if !board.sides[mover as usize].is_piece(&knight_square) {
                knight_moves.push( Move {
                    color: mover,
                    origin: *origin,
                    piece: Piece::Knight,
                    destination: knight_square,
                    promote_type: None,
                    is_castle: false,
                });
            }
        }
        knight_moves
    }
    
    // Returns all valid squares in a poaitive direction.
    pub fn get_positive_ray_attacks(board: &Board, origin: &Square, dir: Direction) -> Bitboard {
        let mover = board.meta.player;
        if dir as usize >= 4 {
            panic!("tried to get_positive_ray_attacks on a negative direction!");
        }
        let mut attacks = RAY_ATTACKS[dir as usize][*origin as usize];
        let all_pieces = board.sides[Color::White as usize].clone().or(&board.sides[Color::Black as usize]);
        let blockers = attacks.clone().and(&all_pieces);
        if blockers.to_integer() != 0 {
            attacks.xor(&RAY_ATTACKS[dir as usize][blockers.find_lsb() as usize]);
            if board.sides[mover as usize].is_piece(&blockers.find_lsb()) {
                attacks.toggle(&blockers.find_lsb());
            }
        }
        attacks
    }
    
    // Returns all valid squares in a positive direction.
    pub fn get_negative_ray_attacks(board: &Board, origin: &Square, dir: Direction) -> Bitboard {
        let mover = board.meta.player;
        if dir as usize <= 3 {
            panic!("tried to get_negative_ray_attacks on a positive direction!");
        }
        let mut attacks = RAY_ATTACKS[dir as usize][*origin as usize];
        let all_pieces = board.sides[Color::White as usize].clone().or(&board.sides[Color::Black as usize]);
        let blockers = attacks.clone().and(&all_pieces);
        if blockers.to_integer() != 0 {
            attacks.xor(&RAY_ATTACKS[dir as usize][blockers.find_msb() as usize]);
            if board.sides[mover as usize].is_piece(&blockers.find_msb()) {
                attacks.toggle(&blockers.find_msb());
            }
        }
        attacks
    }

    // generate_all_bishop_moves does NOT check for legality.
    pub fn generate_all_bishop_moves(board: &Board, origin: &Square) -> Vec<Move> {
        let mover = board.meta.player;
    
        let northwest = Move::get_positive_ray_attacks(board, origin, Direction::Northwest).get_squares();
        let northeast = Move::get_positive_ray_attacks(board, origin, Direction::Northeast).get_squares();
        let southwest = Move::get_negative_ray_attacks(board, origin, Direction::Southwest).get_squares();
        let southeast = Move::get_negative_ray_attacks(board, origin, Direction::Southeast).get_squares();
        
        let mut bishop_squares = [northwest, northeast, southwest, southeast].concat();
        let mut bishop_moves = Vec::new();

        while bishop_squares.len() != 0 {
            bishop_moves.push( Move {
                color: mover,
                origin: *origin,
                piece: Piece::Bishop,
                destination: bishop_squares.pop().unwrap(),
                promote_type: None,
                is_castle: false,
            });
        }
        bishop_moves
    }

    // generate_all_rook_moves does NOT check for legality.
    pub fn generate_all_rook_moves(board: &Board, origin: &Square) -> Vec<Move> {
        let mover = board.meta.player;
    
        let west = Move::get_negative_ray_attacks(board, origin, Direction::West).get_squares();
        let north = Move::get_positive_ray_attacks(board, origin, Direction::North).get_squares();
        let east = Move::get_positive_ray_attacks(board, origin, Direction::East).get_squares();
        let south = Move::get_negative_ray_attacks(board, origin, Direction::South).get_squares();
        
        let mut rook_squares = [west, north, east, south].concat();
        let mut rook_moves = Vec::new();

        while rook_squares.len() != 0 {
            rook_moves.push( Move {
                color: mover,
                origin: *origin,
                piece: Piece::Rook,
                destination: rook_squares.pop().unwrap(),
                promote_type: None,
                is_castle: false,
            });
        }
        rook_moves
    }

    // generate_all_queen_moves does NOT check for legality.
    pub fn generate_all_queen_moves(board: &Board, origin: &Square) -> Vec<Move> {
        let mover = board.meta.player;
    
        let west = Move::get_negative_ray_attacks(board, origin, Direction::West).get_squares();
        let north = Move::get_positive_ray_attacks(board, origin, Direction::North).get_squares();
        let east = Move::get_positive_ray_attacks(board, origin, Direction::East).get_squares();
        let south = Move::get_negative_ray_attacks(board, origin, Direction::South).get_squares();
        let northwest = Move::get_positive_ray_attacks(board, origin, Direction::Northwest).get_squares();
        let northeast = Move::get_positive_ray_attacks(board, origin, Direction::Northeast).get_squares();
        let southwest = Move::get_negative_ray_attacks(board, origin, Direction::Southwest).get_squares();
        let southeast = Move::get_negative_ray_attacks(board, origin, Direction::Southeast).get_squares();
        
        let mut queen_squares = [west, north, east, south, northwest, northeast, southwest, southeast].concat();
        let mut queen_moves = Vec::new();

        while queen_squares.len() != 0 {
            queen_moves.push( Move {
                color: mover,
                origin: *origin,
                piece: Piece::Queen,
                destination: queen_squares.pop().unwrap(),
                promote_type: None,
                is_castle: false,
            });
        }
        queen_moves
    }

    // generate_all_pawn_moves does NOT check for legality.
    pub fn generate_all_pawn_moves(board: &Board, origin: &Square) -> Vec<Move> {
        let mover = board.meta.player;
        let all_pieces = board.sides[Color::White as usize].clone().or(&board.sides[Color::Black as usize]);

        let mut pawn_squares: Vec<Square> = Vec::new();
        let mut pawn_moves: Vec<Move> = Vec::new();

        // Check if white pawn on 2nd rank.
        if mover == Color::White {

            if origin.get_rank() == 2 
            {
                pawn_squares.push(Square::from_int(*origin as usize + 2 * 8));
            }
            let northwest_square = Square::from_int(*origin as usize + 7);
            if origin.get_file() != 0 && 
                board.sides[Color::Black as usize].is_piece(&northwest_square) || 
                board.meta.en_passant_square == Some(northwest_square) 
            {
                pawn_squares.push(northwest_square);
            }
            let northeast_square = Square::from_int(*origin as usize + 9);
            if origin.get_file() != 7 && 
                board.sides[Color::Black as usize].is_piece(&northeast_square) || 
                board.meta.en_passant_square == Some(northeast_square) 
            {
            pawn_squares.push(northeast_square);
            } 
            // One square forward.
            if !all_pieces.is_piece(&Square::from_int(*origin as usize + 8)) 
            {
                pawn_squares.push(Square::from_int(*origin as usize + 8));
            }
        
        } else {
            
            if origin.get_rank() == 7 
            {
                pawn_squares.push(Square::from_int(*origin as usize - 2 * 8));
            }
            let southwest_square = Square::from_int(*origin as usize - 9);
            if origin.get_file() != 0 && 
                board.sides[Color::White as usize].is_piece(&southwest_square) || 
                board.meta.en_passant_square == Some(southwest_square) 
            {
                pawn_squares.push(southwest_square);
            }
            let southeast_square = Square::from_int(*origin as usize - 7);
            if origin.get_file() != 7 && 
                board.sides[Color::White as usize].is_piece(&southeast_square) || 
                board.meta.en_passant_square == Some(southeast_square) 
            {
            pawn_squares.push(southeast_square);
            } 
            // One square forward.
            if !all_pieces.is_piece(&Square::from_int(*origin as usize - 8)) 
            {
                pawn_squares.push(Square::from_int(*origin as usize - 8));
            }
        }

        println!("{:?}", pawn_squares);

        // All non-promotion moves!
        while pawn_squares.len() != 0 {
            // Check if the move is a promotion.
            if (origin.get_rank() == 7 && mover == Color::White) || (origin.get_rank() == 2 && mover == Color::Black) {
                let destination = pawn_squares.pop().unwrap();
                for i in 0..PIECETYPE_COUNT {
                    pawn_moves.push( Move {
                        color: mover,
                        origin: *origin,
                        piece: Piece::Pawn,
                        destination: destination,
                        promote_type: Some(Piece::from_id(i)),
                        is_castle: false,
                    })
                }
            } else {
            pawn_moves.push( Move {
                color: mover,
                origin: *origin,
                piece: Piece::Pawn,
                destination: pawn_squares.pop().unwrap(),
                promote_type: None,
                is_castle: false,
            });
            }
        }
        /*
                    if origin.get_rank() == 7 &&
                !all_pieces.is_piece(&Square::from_int(*origin as usize + 8)) 
            {

            }
         */
        pawn_moves
    }
    
    /*
        pub fn generate_all_king_moves(board: &Board, origin: &Square) -> Vec<Move> {
        let mover = board.meta.player;

    }
     */

    /*
    pub fn get_moves<>
     */
    
}