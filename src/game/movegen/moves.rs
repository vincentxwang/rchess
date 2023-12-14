use crate::core::structs::Direction;
use crate::engine::evaluate::Score;
use crate::engine::evaluate::material::mvv_lva;
use crate::game::piece::Piece as Piece; 
use crate::game::bitboard::Bitboard as Bitboard;
use crate::core::structs::Square as Square;
use crate::core::structs::Color as Color;
use crate::game::board::Board as Board;
use crate::game::movegen::magic::*;

// Move represents a single move from one side on a chessboard. This is otherwise called a "half-move."
#[derive(Debug, Copy, Clone)]
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
            // Ensures that '\n' is not taken as a character!
            if next.is_some_and(|x| x != 0xA as char) {
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

        if piece == Piece::King && Square::horizontal_distance(origin_square, destination_square) > 1 {
            is_castle = true;
        }

        Move {
            color,
            piece,
            origin: origin_square,
            destination: destination_square,
            promote_type: promotion,
            is_castle,
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

        while let Some(knight_square) = knight_squares.pop() {
            
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
    pub fn get_positive_ray_attacks(board: &Board, origin: &Square, dir: Direction, color: Color) -> Bitboard {
        let mover = color;
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
    pub fn get_negative_ray_attacks(board: &Board, origin: &Square, dir: Direction, color: Color) -> Bitboard {
        let mover = color;
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
    pub fn generate_all_bishop_moves(board: &Board, origin: &Square, color: Color) -> Vec<Move> {
        let mover = board.meta.player;
    
        let northwest = Move::get_positive_ray_attacks(board, origin, Direction::Northwest, color).get_squares();
        let northeast = Move::get_positive_ray_attacks(board, origin, Direction::Northeast, color).get_squares();
        let southwest = Move::get_negative_ray_attacks(board, origin, Direction::Southwest, color).get_squares();
        let southeast = Move::get_negative_ray_attacks(board, origin, Direction::Southeast, color).get_squares();
        
        let mut bishop_squares = [northwest, northeast, southwest, southeast].concat();
        let mut bishop_moves = Vec::new();

        while !bishop_squares.is_empty() {
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
    pub fn generate_all_rook_moves(board: &Board, origin: &Square, color: Color) -> Vec<Move> {
        let mover = board.meta.player;
    
        let west = Move::get_negative_ray_attacks(board, origin, Direction::West, color).get_squares();
        let north = Move::get_positive_ray_attacks(board, origin, Direction::North, color).get_squares();
        let east = Move::get_positive_ray_attacks(board, origin, Direction::East, color).get_squares();
        let south = Move::get_negative_ray_attacks(board, origin, Direction::South, color).get_squares();
        
        let mut rook_squares = [west, north, east, south].concat();
        let mut rook_moves = Vec::new();

        while !rook_squares.is_empty() {
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
    pub fn generate_all_queen_moves(board: &Board, origin: &Square, color: Color) -> Vec<Move> {
        let mover = board.meta.player;
    
        let west = Move::get_negative_ray_attacks(board, origin, Direction::West, color).get_squares();
        let north = Move::get_positive_ray_attacks(board, origin, Direction::North, color).get_squares();
        let east = Move::get_positive_ray_attacks(board, origin, Direction::East, color).get_squares();
        let south = Move::get_negative_ray_attacks(board, origin, Direction::South, color).get_squares();
        let northwest = Move::get_positive_ray_attacks(board, origin, Direction::Northwest, color).get_squares();
        let northeast = Move::get_positive_ray_attacks(board, origin, Direction::Northeast, color).get_squares();
        let southwest = Move::get_negative_ray_attacks(board, origin, Direction::Southwest, color).get_squares();
        let southeast = Move::get_negative_ray_attacks(board, origin, Direction::Southeast, color).get_squares();
        
        let mut queen_squares = [west, north, east, south, northwest, northeast, southwest, southeast].concat();
        let mut queen_moves = Vec::new();

        while !queen_squares.is_empty() {
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
            if origin.get_rank() == 2 && 
                !all_pieces.is_piece(&Square::from_int(*origin as usize + 8)) &&
                !all_pieces.is_piece(&Square::from_int(*origin as usize + 16))
            {
                pawn_squares.push(Square::from_int(*origin as usize + 2 * 8));
            }
            let northwest_square = Square::from_int(*origin as usize + 7);
            if origin.get_file() != 0 && 
                (board.sides[Color::Black as usize].is_piece(&northwest_square) || 
                board.meta.en_passant_square == Some(northwest_square))
            {
                pawn_squares.push(northwest_square);
            }
            let northeast_square = Square::from_int(*origin as usize + 9);
            if origin.get_file() != 7 && 
                (board.sides[Color::Black as usize].is_piece(&northeast_square) || 
                board.meta.en_passant_square == Some(northeast_square))
            {
            pawn_squares.push(northeast_square);
            } 
            // One square forward.
            if !all_pieces.is_piece(&Square::from_int(*origin as usize + 8)) 
            {
                pawn_squares.push(Square::from_int(*origin as usize + 8));
            }
        
        } else {
            
            if origin.get_rank() == 7 &&
            !all_pieces.is_piece(&Square::from_int(*origin as usize - 8)) &&
            !all_pieces.is_piece(&Square::from_int(*origin as usize - 16))
            {
                pawn_squares.push(Square::from_int(*origin as usize - 2 * 8));
            }
            let southwest_square = Square::from_int(*origin as usize - 9);
            if origin.get_file() != 0 && 
                (board.sides[Color::White as usize].is_piece(&southwest_square) || 
                board.meta.en_passant_square == Some(southwest_square))
            {
                pawn_squares.push(southwest_square);
            }
            let southeast_square = Square::from_int(*origin as usize - 7);
            if origin.get_file() != 7 && 
                (board.sides[Color::White as usize].is_piece(&southeast_square) || 
                board.meta.en_passant_square == Some(southeast_square))
            {
                pawn_squares.push(southeast_square);
            } 
            // One square forward.
            if !all_pieces.is_piece(&Square::from_int(*origin as usize - 8)) 
            {
                pawn_squares.push(Square::from_int(*origin as usize - 8));
            }
        }

        while !pawn_squares.is_empty() {
            // Check if the move is a promotion.
            if (origin.get_rank() == 7 && mover == Color::White) || (origin.get_rank() == 2 && mover == Color::Black) {
                let destination = pawn_squares.pop().unwrap();
                for i in [
                    Piece::Knight,
                    Piece::Bishop,
                    Piece::Rook,
                    Piece::Queen,
                ] {
                    pawn_moves.push( Move {
                        color: mover,
                        origin: *origin,
                        piece: Piece::Pawn,
                        destination,
                        promote_type: Some(i),
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
    
    // generate_all_king_moves does NOT check for legality. Castling handled separately.
    pub fn generate_all_king_moves(board: &Board, origin: &Square) -> Vec<Move> {
        let mover = board.meta.player;
        let mut king_squares: Vec<Square> = Vec::new();

        let file = origin.get_file();
        let rank = origin.get_rank();

        // E
        if file <= 6 {
            king_squares.push(Square::from_int(*origin as usize + 1));
        } 

        // NE
        if file <= 6 && rank <= 7 {
            king_squares.push(Square::from_int(*origin as usize + 9));
        }

        // N
        if rank <= 7 {
            king_squares.push(Square::from_int(*origin as usize + 8));
        }

        // NW
        if file >= 1 && rank <= 7 {
            king_squares.push(Square::from_int(*origin as usize + 7));
        }

        // W
        if file >= 1 {
            king_squares.push(Square::from_int(*origin as usize - 1));
        }

        // SW
        if file >= 1 && rank >= 2 {
            king_squares.push(Square::from_int(*origin as usize - 9));
        }

        // S
        if rank >= 2 {
            king_squares.push(Square::from_int(*origin as usize - 8));
        }

        // NNW
        if file <= 6 && rank >= 2 {
            king_squares.push(Square::from_int(*origin as usize - 7));
        }

        let mut king_moves = Vec::new();

        while let Some(king_square) = king_squares.pop() {
            
            if !board.sides[mover as usize].is_piece(&king_square) {
                king_moves.push( Move {
                    color: mover,
                    origin: *origin,
                    piece: Piece::King,
                    destination: king_square,
                    promote_type: None,
                    is_castle: false,
                });
            }
        }
        king_moves
    }
    
    // generate_castles does check for legality.
    pub fn generate_castles(board: &Board) -> Vec<Move> {

        let mover = board.meta.player;
        
        let mut castle_moves = Vec::new();
        let king_square;
        let kingside_index;
        let queenside_index;

        if mover == Color::White {
            king_square = Square::E1;
            kingside_index = 0;
            queenside_index = 1;
        } else {
            king_square = Square::E8;
            kingside_index = 2;
            queenside_index = 3;
        }

        if board.meta.castle_rights[kingside_index] 
            && !board.is_attacked(&king_square, mover) 
            && !board.is_attacked(&Square::from_int(king_square as usize + 1), mover)
            && (board.get_piece(&Square::from_int(king_square as usize + 1)).is_none())
            && !board.is_attacked(&Square::from_int(king_square as usize + 2), mover)
            && (board.get_piece(&Square::from_int(king_square as usize + 2)).is_none()) {
            castle_moves.push( Move {
                color: board.meta.player,
                origin: king_square,
                piece: Piece::King,
                destination: Square::from_int(king_square as usize + 2),
                promote_type: None,
                is_castle: true,
            })
        } 
        
        if board.meta.castle_rights[queenside_index] 
            && !board.is_attacked(&king_square, mover) 
            && !board.is_attacked(&Square::from_int(king_square as usize - 1), mover)
            && (board.get_piece(&Square::from_int(king_square as usize - 1)).is_none())
            && !board.is_attacked(&Square::from_int(king_square as usize - 2), mover) 
            && (board.get_piece(&Square::from_int(king_square as usize - 2)).is_none()) 
            // Note that b8 being under attack is ok to castle still.
            && (board.get_piece(&Square::from_int(king_square as usize - 3)).is_none()) {
            castle_moves.push( Move {
                color: board.meta.player,
                origin: king_square,
                piece: Piece::King,
                destination: Square::from_int(king_square as usize - 2),
                promote_type: None,
                is_castle: true,
            })
        } 
        castle_moves
    }

    // generate_all_moves() will likely contain moves that are illegal. It will also order captures first by MVV - LVA.
    pub fn generate_all_moves(board: &Board) -> Vec<Move> {
        let mut all_moves = Vec::new();
        let mover = board.meta.player;

        // Castling is added before the others because castling is usually a good move.
        all_moves.append(&mut Move::generate_castles(board));

        for i in 0..64 {
            let sq = Square::from_int(i);
            let piece = board.get_piece(&sq);
            if piece.is_some() && piece.unwrap().1 == mover {
                let mut piece_moves = match piece.unwrap().0 {
                    Piece::Pawn => Move::generate_all_pawn_moves(board, &sq),
                    Piece::Knight => Move::generate_all_knight_moves(board, &sq),
                    Piece::Bishop => Move::generate_all_bishop_moves(board, &sq, mover),
                    Piece::Rook => Move::generate_all_rook_moves(board, &sq, mover),
                    Piece::Queen => Move::generate_all_queen_moves(board, &sq, mover),
                    Piece::King => Move::generate_all_king_moves(board, &sq),
                };
                all_moves.append(&mut piece_moves);
            }
        }

        all_moves.sort_by(|x, y| {
            let mut board_x = board.clone();
            board_x.process_move(x);
            let score_x = Score::get_score(&board_x);
            let mut board_y= board.clone();
            board_y.process_move(y);
            let score_y = Score::get_score(&board_y);
            score_y.cmp(&score_x)
        }); 

        all_moves
    }

    pub fn generate_legal_moves(board: &Board) -> Vec<Move> {
        let mut moves = Self::generate_all_moves(board);

        for i in (0..(moves.len())).rev() {
            let mut board_state = *board;
            if board_state.process_move(&moves[i]).is_err() {
                moves.remove(i);
            }
        }
        moves
    }

    pub fn generate_random_move(board: &Board) -> Move {
        let legal_moves = Move::generate_legal_moves(board);
        let rand = rand::random::<u64>() % legal_moves.len() as u64;
        legal_moves[rand as usize]
    }
}