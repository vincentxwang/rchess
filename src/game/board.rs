use crate::core::structs::Direction;
use crate::game::bitboard::Bitboard as Bitboard;
use crate::core::constants::*;
use crate::core::structs::Color as Color;
use crate::core::structs::Square as Square;
use crate::game::moves::Move as Move;
use super::piece::Piece;

#[derive(Debug)]
pub struct Board {
    
    // Bitboards representing the white/black pieces. [White, Black].
    pub sides: [Bitboard; PLAYER_COUNT],
    // Bitboards representing piece types. [Pawn, Knight, Bishop, Rook, Queen, King].
    pub pieces: [Bitboard; PIECETYPE_COUNT],
    // Keeps a meta of BoardData.
    pub meta: BoardData
    // pub moves: Vec<Option<(Move, Option<Piece>)>>
}

// BoardData stores additional information about the Board aside from piece locations.
#[derive(Debug)]
pub struct BoardData {
    pub player: Color,
    pub castle_rights: CastleRights,
    pub fifty_move: u8, 
    pub en_passant_square: Option<Square>,
    pub full_moves: u8,
}

impl Board {
    // Constructs a new game.
    pub fn new() -> Board {
        Board {
            sides: [
                Bitboard::new(WHITE_START),
                Bitboard::new(BLACK_START),
            ],
            pieces: [
                Bitboard::new(PAWN_START),
                Bitboard::new(KNIGHT_START),
                Bitboard::new(BISHOP_START),
                Bitboard::new(ROOK_START),
                Bitboard::new(QUEEN_START),
                Bitboard::new(KING_START),
            ],
            meta: BoardData {
                player: Color::White,
                castle_rights: CastleRights {
                    white_kingside: true,
                    white_queenside: true,
                    black_kingside: true,
                    black_queenside: true,
                },
                fifty_move: 0,
                en_passant_square: None,
                full_moves: 0,
            }
        }
    }

    pub fn empty() -> Board {
        Board {
            sides: [
                Bitboard::empty(),
                Bitboard::empty(),
            ],
            pieces: [
                Bitboard::empty(),
                Bitboard::empty(),
                Bitboard::empty(),
                Bitboard::empty(),
                Bitboard::empty(),
                Bitboard::empty(),
            ],
            meta: BoardData {
                player: Color::White,
                castle_rights: CastleRights {
                    white_kingside: false,
                    white_queenside: false,
                    black_kingside: false,
                    black_queenside: false,
                },
                fifty_move: 0,
                en_passant_square: None,
                full_moves: 0,
            }
        }
    }

    // Adds a piece to the board.
    // remember to pass by reference because there is no reason to take ownership of the data here...
    // making the sides and pieces attributes in Board makes this function much easier to work with! :)
    pub fn add_piece(&mut self, color: &Color, piece: &Piece, sq: &Square) {
        self.sides[*color as usize].insert(sq); 
        self.pieces[*piece as usize].insert(sq);
    }

    // Constructs the board from a FEN string.
    pub fn from_fen(fen: &str) -> Result<Board, &str> {

        let mut board = Board {
            sides: [
                Bitboard::empty(),
                Bitboard::empty(),
            ],
            pieces: [
                Bitboard::empty(),
                Bitboard::empty(),
                Bitboard::empty(),
                Bitboard::empty(),
                Bitboard::empty(),
                Bitboard::empty(),
            ],
            meta: BoardData {
                player: Color::White,
                castle_rights: CastleRights {
                    white_kingside: false,
                    white_queenside: false,
                    black_kingside: false,
                    black_queenside: false,
                },
                fifty_move: 0,
                en_passant_square: None,
                full_moves: 0,
            }
        };

        // Chars<'_> represents an iterator!
        let mut fen_chars = fen.chars();
        let mut row: u8 = 7;
        let mut col: u8 = 0;

        while true {
            let chr = fen_chars
                .next()
                .ok_or("incomplete FEN string");
            let color = if chr?.is_uppercase() {
                Color::White 
            } else {
                Color::Black
            };

            if chr.unwrap().is_ascii_digit() {
                if chr.unwrap().to_digit(10).unwrap() > 0 && chr.unwrap().to_digit(10).unwrap() <= 8 {
                    col += chr.unwrap().to_digit(10).unwrap() as u8 - 1;
                    if col > 8 {
                        panic!("too many squares skipped in FEN string");
                    }
                }
            } else {
                let piece = Piece::from_code(chr.unwrap().to_ascii_uppercase());
                let sq = Square::from_int((row * 8 + col) as usize);
                board.add_piece(&color, &piece, &sq);
            }
            if col == 7 && row != 0 {
                if fen_chars.next().ok_or("incomplete FEN string") == Ok('/') {
                    col = 0;
                    row -= 1;
                } else {
                    panic!("missing '/' in between lines");
                }
            } else if col == 7 && row == 0 {
                break;
            } else {
                col += 1;
            }
        }

        if fen_chars.next().ok_or("incomplete FEN string") != Ok(' ') {
            panic!("expected space after board array")
        }

        board.meta.player = match fen_chars.next().ok_or("incomplete FEN string") {
            Ok('w') => Color::White,
            Ok('b') => Color::Black,
            _ => panic!("invalid player!")
        };

        if fen_chars.next().ok_or("incomplete FEN string").unwrap() != ' ' {
            panic!("expected space after player turn indicator")
        }

        let mut chr = fen_chars.next().ok_or("incomplete FEN string").unwrap();

        while chr != ' ' {
            match chr {
                'q' => board.meta.castle_rights.black_queenside = true,
                'Q' => board.meta.castle_rights.white_queenside = true,
                'k' => board.meta.castle_rights.black_kingside = true,
                'K' => board.meta.castle_rights.white_kingside = true,
                '-' => (),
                _ => panic!("invalid castling characters")
            }
            chr = fen_chars.next().ok_or("incomplete FEN string").unwrap();
        }

        // we do not need to check for a space because the code above consumes it

        board.meta.en_passant_square = {
            let ep_file = fen_chars
                .next()
                .ok_or("reached end while parsing en passant file");
            if ep_file == Ok('-') {
                None
            } else {
                let ep_rank = fen_chars.next().ok_or("reached end while parsing en passant rank").unwrap();
                let ep_square = ep_file.unwrap().to_string() + &ep_rank.to_string();
                Some(Square::from_str(ep_square))
            }
       };

        if fen_chars.next().ok_or("incomplete FEN string").unwrap() != ' ' {
            panic!("expected space after en passant square")
        }


        // technically we allow for fifty_move values greater than 50, but less than 100.
        board.meta.fifty_move = {
            let digit_1: u8 = fen_chars.next().ok_or("incomplete FEN string").unwrap().to_digit(10).unwrap() as u8;
            let possibly_digit_2 = fen_chars.next().ok_or("incomplete FEN string").unwrap();
    
            if possibly_digit_2 == ' ' {
                digit_1
            } else if possibly_digit_2.is_ascii_digit() {
                if fen_chars.next().ok_or("incomplete FEN string").unwrap() != ' ' {
                    panic!("expected space after 50 move");
                }
                digit_1 * 10 + possibly_digit_2.to_digit(10).unwrap() as u8
            } else {
                panic!("expected number as second digit of 50 move");
            }
        };

        board.meta.full_moves = {
            let digit_1: u8 = fen_chars.next().ok_or("incomplete FEN string").unwrap().to_digit(10).unwrap() as u8;
            let possibly_digit_2 = fen_chars.next();
    
            if possibly_digit_2 == None {
                digit_1
            } else if possibly_digit_2.unwrap().is_ascii_digit() {
                digit_1 * 10 + possibly_digit_2.unwrap().to_digit(10).unwrap() as u8
            } else {
                panic!("expected number as second digit of move counter");
            }
        };

        Ok(board)
    }  

    // Gets a piece from a square on a board.
    pub fn get_piece(&self, sq: &Square) -> Option<(Piece, Color)> {
        if !self.sides[Color::White as usize].is_piece(sq) && !self.sides[Color::Black as usize].is_piece(sq) {
            return None;
        } 
        let color = {
            if self.sides[Color::White as usize].is_piece(sq) {
                Color::White
            } else {
                Color::Black
            }
        };
  
        for i in 0..PIECETYPE_COUNT {
            if self.pieces[i].is_piece(sq) {
                return Some((Piece::from_id(i), color));
            }
        }

        panic!("something went wrong with get_piece. particularly a piece was detected in sides but not in pieces. bad.")
    }

    pub fn print_board(&self) {
        println!("--------- Printing Board ----------");
        for rank in (0..8).rev() {
            for file in 0..8 {
                let sq = Square::from_int(8 * rank + file);
                let piece = self.get_piece(&sq);
                if piece.is_none() {
                    print!("- ");
                } else {
                    // Note that Piece::code returns an upper case letter.
                    let char = Piece::code(piece.unwrap().0);
                    if piece.unwrap().1 == Color::Black {
                        print!("{} ", char.to_ascii_lowercase());
                    } else {
                        print!("{} ", char);
                    }
                }
            }
            println!("");
        }
        println!("----------- End of Print ------------");
    }

    // Gets the furthest piece along an attack ray in a direction. 
    // Note that this will return None if the piece encountered is of the same Color.
    pub fn get_furthest_piece_along_ray(&self, sq: &Square, dir: Direction) -> Option<(Piece, Color)> {

        // Positive rays.
        if dir as usize <= 3 {
            let bitboard = Move::get_positive_ray_attacks(self, &sq, dir);
            if bitboard.to_integer() == 0 {
                return None;
            } 
            return self.get_piece(&bitboard.find_msb());
        // Negative rays.
        } else {
            let bitboard = Move::get_negative_ray_attacks(self, &sq, dir);
            if bitboard.to_integer() == 0 {
                return None;
            }
            return self.get_piece(&bitboard.find_lsb());
        }
    }

    // Checks if a square is attacked. Very naive. Possibly fails.
    pub fn is_attacked(&self, sq: &Square) -> bool {
        let mover = self.meta.player;
        let not_mover = Color::not(mover);

        let file = sq.get_file();
        let rank = sq.get_rank();
        // Check for pawn.
        if mover == Color::White {
            if rank != 8 &&
                // We put parentheses here so the bottom is not evaluated when "rank != 8" short circuits.
                (self.get_piece(&Square::from_int(*sq as usize + 7)) == Some((Piece::Pawn, not_mover)) ||
                self.get_piece(&Square::from_int(*sq as usize + 9)) == Some((Piece::Pawn, not_mover)))
            {
                return true;
            }
        } else {
            if rank != 1 &&
                (self.get_piece(&Square::from_int(*sq as usize - 7)) == Some((Piece::Pawn, not_mover)) ||
                self.get_piece(&Square::from_int(*sq as usize - 9)) == Some((Piece::Pawn, not_mover)))
            {
                return true;
            }
        }
        // Check for knight.
        // NWW
        if file >= 2 && rank <= 7 &&
            self.get_piece(&Square::from_int(*sq as usize + 6)) == Some((Piece::Knight, not_mover))
        {
            return true;
        } 

        // SWW
        if file >= 2 && rank >= 2 &&
            self.get_piece(&Square::from_int(*sq as usize - 10)) == Some((Piece::Knight, not_mover))
        {
            return true;
        } 

        // SSW
        if file >= 1 && rank >= 3 &&
            self.get_piece(&Square::from_int(*sq as usize - 17)) == Some((Piece::Knight, not_mover))
        {
            return true;
        } 

        // SSE
        if file <= 6 && rank >= 3 &&
            self.get_piece(&Square::from_int(*sq as usize - 15)) == Some((Piece::Knight, not_mover))
        {
            return true;
        } 

        // SEE
        if file <= 5 && rank >= 2 &&
            self.get_piece(&Square::from_int(*sq as usize - 6)) == Some((Piece::Knight, not_mover))
        {
            return true;
        } 

        // NEE
        if file <= 5 && rank <= 7 &&
            self.get_piece(&Square::from_int(*sq as usize + 10)) == Some((Piece::Knight, not_mover))
        {
            return true;
        } 

        // NNE
        if file <= 6 && rank <= 6 &&
            self.get_piece(&Square::from_int(*sq as usize + 17)) == Some((Piece::Knight, not_mover))
        {
            return true;
        } 
        
        // NNW
        if file >= 1 && rank <= 6 &&
            self.get_piece(&Square::from_int(*sq as usize + 15)) == Some((Piece::Knight, not_mover))
        {
            return true;
        } 

        // Check for bishop.
        let possible_bishop_threats = vec![
            self.get_furthest_piece_along_ray(sq, Direction::Northwest),
            self.get_furthest_piece_along_ray(sq, Direction::Northeast),
            self.get_furthest_piece_along_ray(sq, Direction::Southeast),
            self.get_furthest_piece_along_ray(sq, Direction::Southwest),
        ];
        if possible_bishop_threats.contains(&Some((Piece::Bishop, not_mover)))
        {
            return true;
        }

        // Check for rook.
        let possible_rook_threats = vec![
            self.get_furthest_piece_along_ray(sq, Direction::North),
            self.get_furthest_piece_along_ray(sq, Direction::East),
            self.get_furthest_piece_along_ray(sq, Direction::South),
            self.get_furthest_piece_along_ray(sq, Direction::West),
        ];
        if possible_rook_threats.contains(&Some((Piece::Rook, not_mover)))
        {
            return true;
        }
        
        // Check for queen.
        let possible_queen_threats = vec![
            self.get_furthest_piece_along_ray(sq, Direction::North),
            self.get_furthest_piece_along_ray(sq, Direction::East),
            self.get_furthest_piece_along_ray(sq, Direction::South),
            self.get_furthest_piece_along_ray(sq, Direction::West),
            self.get_furthest_piece_along_ray(sq, Direction::Northwest),
            self.get_furthest_piece_along_ray(sq, Direction::Northeast),
            self.get_furthest_piece_along_ray(sq, Direction::Southeast),
            self.get_furthest_piece_along_ray(sq, Direction::Southwest),
        ];
        if possible_queen_threats.contains(&Some((Piece::Queen, not_mover)))
        {
            return true;
        }

        false
    }
}

#[derive(Debug)]
pub struct CastleRights {
    // boolean is bool, not Boolean!
    white_kingside: bool,
    white_queenside: bool,
    black_kingside: bool,
    black_queenside: bool,
}