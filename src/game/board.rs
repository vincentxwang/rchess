use crate::core::structs::Direction;
use crate::engine::zobrist::Zobrist;
use crate::game::bitboard::Bitboard as Bitboard;
use crate::core::constants::*;
use crate::core::structs::Color as Color;
use crate::core::structs::Square as Square;
use crate::game::movegen::moves::Move as Move;
use super::piece::Piece;
use crate::engine::zobrist::*;

#[derive(Debug, Copy, Clone)]
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
#[derive(Debug, Copy, Clone)]
pub struct BoardData {
    pub player: Color,
    pub castle_rights: [bool; 4], // White kingside, White queenside, Black kingside, Black queenside
    pub fifty_move: u8, 
    pub en_passant_square: Option<Square>,
    pub full_moves: u8,
    pub zobrist: Zobrist,
}

impl Default for Board {
    fn default() -> Self {
        Self::new()
    }
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
                castle_rights: [true, true, true, true],
                // These are half moves.
                fifty_move: 0,
                en_passant_square: None,
                full_moves: 1,
                // Number obtained from running zobrist_hash on Board::new()
                zobrist: Zobrist(15988586886729190057),
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
                castle_rights: [false; 4],
                fifty_move: 0,
                en_passant_square: None,
                full_moves: 1,
                zobrist: Zobrist(0),
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
                castle_rights: [false; 4],
                fifty_move: 0,
                en_passant_square: None,
                full_moves: 0,
                // Temporarily, this is 0. We update it at the end.
                zobrist: Zobrist(0),
            }
        };

        // Chars<'_> represents an iterator!
        let mut fen_chars = fen.chars();
        let mut row: u8 = 7;
        let mut col: u8 = 0;

        loop {
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
                'q' => board.meta.castle_rights[3] = true,
                'Q' => board.meta.castle_rights[1] = true,
                'k' => board.meta.castle_rights[2] = true,
                'K' => board.meta.castle_rights[0]= true,
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
    
            if possibly_digit_2.is_none() {
                digit_1
            } else if possibly_digit_2.unwrap().is_ascii_digit() {
                digit_1 * 10 + possibly_digit_2.unwrap().to_digit(10).unwrap() as u8
            } else {
                panic!("expected number as second digit of move counter");
            }
        };

        board.meta.zobrist = Zobrist::zobrist_hash(&board);

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

    // Prints a representation of the board in terminal.
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
            println!();
        }
        println!("Player: {:?}", self.meta.player);
        println!("Castle Rights: {:?}", self.meta.castle_rights);
        println!("Fifty Move: {:?}", self.meta.fifty_move);
        println!("En Passant Square: {:?}", self.meta.en_passant_square);
        println!("Full Move: {:?}", self.meta.full_moves);
        println!("----------- End of Print ------------");
    }

    // Gets square of king of a color. Assumes there is one and only one.
    pub fn get_king(&self, color: &Color) -> Square {
        self.sides[*color as usize].clone().and(&self.pieces[Piece::King as usize]).find_lsb()
    }

    // Gets the furthest piece along an attack ray in a direction. 
    // Note that this will return None if the piece encountered is of the same Color.
    pub fn get_furthest_piece_along_ray(&self, sq: &Square, dir: Direction, color: Color) -> Option<(Piece, Color)> {

        // Positive rays.
        if dir as usize <= 3 {
            let bitboard = Move::get_positive_ray_attacks(self, sq, dir, color);
            if bitboard.to_integer() == 0 {
                return None;
            } 
            self.get_piece(&bitboard.find_msb())
        // Negative rays.
        } else {
            let bitboard = Move::get_negative_ray_attacks(self, sq, dir, color);
            if bitboard.to_integer() == 0 {
                return None;
            }
            self.get_piece(&bitboard.find_lsb())
        }
    }

    // Checks if a square is attacked. Very naive.
    pub fn is_attacked(&self, sq: &Square, color: Color) -> bool {
        let mover = color;
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
        } else if rank != 1 &&
            (self.get_piece(&Square::from_int(*sq as usize - 7)) == Some((Piece::Pawn, not_mover)) ||
            self.get_piece(&Square::from_int(*sq as usize - 9)) == Some((Piece::Pawn, not_mover)))
        {
                return true;
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
        let possible_bishop_threats = [
            self.get_furthest_piece_along_ray(sq, Direction::Northwest, color),
            self.get_furthest_piece_along_ray(sq, Direction::Northeast, color),
            self.get_furthest_piece_along_ray(sq, Direction::Southeast, color),
            self.get_furthest_piece_along_ray(sq, Direction::Southwest, color),
        ];
        if possible_bishop_threats.contains(&Some((Piece::Bishop, not_mover)))
        {
            return true;
        }

        // Check for rook.
        let possible_rook_threats = [
            self.get_furthest_piece_along_ray(sq, Direction::North, color),
            self.get_furthest_piece_along_ray(sq, Direction::East, color),
            self.get_furthest_piece_along_ray(sq, Direction::South, color),
            self.get_furthest_piece_along_ray(sq, Direction::West, color),
        ];
        if possible_rook_threats.contains(&Some((Piece::Rook, not_mover)))
        {
            return true;
        }
        
        // Check for queen.
        let possible_queen_threats = [
            self.get_furthest_piece_along_ray(sq, Direction::North, color),
            self.get_furthest_piece_along_ray(sq, Direction::East, color),
            self.get_furthest_piece_along_ray(sq, Direction::South, color),
            self.get_furthest_piece_along_ray(sq, Direction::West, color),
            self.get_furthest_piece_along_ray(sq, Direction::Northwest, color),
            self.get_furthest_piece_along_ray(sq, Direction::Northeast, color),
            self.get_furthest_piece_along_ray(sq, Direction::Southeast, color),
            self.get_furthest_piece_along_ray(sq, Direction::Southwest, color),
        ];
        if possible_queen_threats.contains(&Some((Piece::Queen, not_mover)))
        {
            return true;
        }

        false
    }

    // Uses a move to update the Zobrist hash.
    pub fn update_zobrist_hash(&mut self, move_played: &Move) {
        let moved_piece = self.get_piece(&move_played.origin).unwrap();
        let captured_piece = self.get_piece(&move_played.destination);

        self.meta.zobrist.0 ^= ZOBRIST_TABLE[moved_piece.1 as usize][moved_piece.0 as usize][move_played.origin as usize];
        self.meta.zobrist.0 ^= ZOBRIST_TABLE[moved_piece.1 as usize][moved_piece.0 as usize][move_played.destination as usize];

        if captured_piece.is_some() {
            let captured_piece = captured_piece.unwrap();
            self.meta.zobrist.0 ^= ZOBRIST_TABLE[captured_piece.1 as usize][captured_piece.0 as usize][move_played.destination as usize];
        }
    }

    // Progresses the state of the game by a half-move. Returns Ok(()) if move is legal, and Err(()) if move is not legal.
    pub fn process_move(&mut self, half_move: &Move) -> Result<(), ()> {
        if half_move.color != self.meta.player {
            panic!("move color disagrees with board player color!")
        }

        let board_copy = *self;

        // Process Zobrist hashing.
        self.update_zobrist_hash(half_move);

        let mover = half_move.color;
        let not_mover = Color::not(mover);
        
        // Process bitboards.
        let captured = self.get_piece(&half_move.destination);
        self.sides[not_mover as usize].set_zero(&half_move.destination);
        if captured.is_some() {
            self.pieces[captured.unwrap().0 as usize].set_zero(&half_move.destination);
        }
        self.sides[mover as usize].switch(&half_move.origin, &half_move.destination);
        self.pieces[half_move.piece as usize].switch(&half_move.origin, &half_move.destination);

        if half_move.promote_type.is_some() {
            self.pieces[Piece::Pawn as usize].set_zero(&half_move.destination);
            self.pieces[half_move.promote_type.unwrap() as usize].set_one(&half_move.destination);
        }

        if half_move.is_castle {
            match half_move.destination {
                Square::G1 => {
                    self.pieces[Piece::Rook as usize].switch(&Square::H1, &Square::F1);
                    self.sides[mover as usize].switch(&Square::H1, &Square::F1);
                },
                Square::C1 => {
                    self.pieces[Piece::Rook as usize].switch(&Square::A1, &Square::D1);
                    self.sides[mover as usize].switch(&Square::A1, &Square::D1);
                }
                Square::G8 => {
                    self.pieces[Piece::Rook as usize].switch(&Square::H8, &Square::F8);
                    self.sides[mover as usize].switch(&Square::H8, &Square::F8);
                },
                Square::C8 => {
                    self.pieces[Piece::Rook as usize].switch(&Square::A8, &Square::D8);
                    self.sides[mover as usize].switch(&Square::A8, &Square::D8);
                },
                _ => panic!("bad castle!!!")
            } 
        }

        if self.meta.en_passant_square.is_some() && half_move.piece == Piece::Pawn && self.meta.en_passant_square.unwrap() == half_move.destination {
            if mover == Color::White {
                self.pieces[Piece::Pawn as usize].set_zero(&Square::from_int(half_move.destination as usize - 8));
                self.sides[not_mover as usize].set_zero(&Square::from_int(half_move.destination as usize - 8));
            } else {
                self.pieces[Piece::Pawn as usize].set_zero(&Square::from_int(half_move.destination as usize + 8));
                self.sides[not_mover as usize].set_zero(&Square::from_int(half_move.destination as usize + 8));
            }
        }

        // Process meta.
        if half_move.origin == Square::H1 || half_move.destination == Square::H1 {
            self.meta.castle_rights[0] = false;
        } else if half_move.origin == Square::A1 || half_move.destination == Square::A1 {
            self.meta.castle_rights[1] = false;
        } else if half_move.origin == Square::H8 || half_move.destination == Square::H8 {
            self.meta.castle_rights[2] = false;
        } else if half_move.origin == Square::A8 || half_move.destination == Square::A8 {
            self.meta.castle_rights[3] = false;
        }

        if half_move.piece == Piece::King {
            if mover == Color::White {
                self.meta.castle_rights[0] = false;
                self.meta.castle_rights[1] = false;
            } else {
                self.meta.castle_rights[2] = false;
                self.meta.castle_rights[3] = false;
            }
        }

        self.meta.player = not_mover;

        if captured.is_none() {
            self.meta.fifty_move += 1;
        } else {
            self.meta.fifty_move = 0;
        }

        if mover == Color::Black {
            self.meta.full_moves += 1;
        }


        if half_move.piece == Piece::Pawn {
            if mover == Color::White &&
                half_move.origin.get_rank() == 2 &&
                half_move.destination.get_rank() == 4 
            {
                self.meta.en_passant_square = Some(Square::from_int(half_move.origin as usize + 8));
            } else if mover == Color::Black &&
            half_move.origin.get_rank() == 7 &&
            half_move.destination.get_rank() == 5
            {
            self.meta.en_passant_square = Some(Square::from_int(half_move.origin as usize - 8));
            } else {
                self.meta.en_passant_square = None;
            }
        } else {
            self.meta.en_passant_square = None;
        }

        // Legality check -- is the king in check after the player's move?
        if self.is_attacked(&self.get_king(&mover), mover) {
            *self = board_copy;
            Err(())
        } else {
            Ok(())
        }
    }
}