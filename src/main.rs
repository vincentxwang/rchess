#[macro_use]
extern crate lazy_static;
use crate::game::bitboard::Bitboard;
use crate::game::board::Board;
pub mod game;
pub mod core;
pub mod tests;
use crate::game::magic::*;
use crate::core::structs::Direction as Direction;
use crate::game::moves::Move as Move;
use crate::core::structs::Square as Square;
use crate::core::structs::Color as Color;
use crate::game::piece::Piece as Piece;

fn main() {

    let mut test1 = Board::from_fen("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1").unwrap();
    //println!("{:?}", Move::generate_all_bishop_moves(&test1, &Square::G2));

    test1.print_board();
    //println!("{:?}", Move::generate_all_queen_moves(&test1, &Square::D1));
    //println!("{:?}", Move::generate_all_pawn_moves(&test1, &Square::G7));
    //println!("{:?}", Move::generate_legal_moves(&test1));
    //println!("{:?}", Move::generate_legal_moves(&test1).len());
    
    
    test1.process_move(&Move {
        color: Color::White,
        piece: Piece::Pawn,
        origin: Square::D2,
        destination: Square::D4,
        promote_type: None,
        is_castle: false,
    });
    
    test1.print_board();

    test1.process_move(&Move {
        color: Color::Black,
        piece: Piece::Pawn,
        origin: Square::G7,
        destination: Square::G6,
        promote_type: None,
        is_castle: false,
    });

    test1.print_board();

    test1.process_move(&Move {
        color: Color::White,
        piece: Piece::Pawn,
        origin: Square::D4,
        destination: Square::D5,
        promote_type: None,
        is_castle: false,
    });

    test1.print_board();

    test1.process_move(&Move {
        color: Color::Black,
        piece: Piece::Knight,
        origin: Square::G8,
        destination: Square::H6,
        promote_type: None,
        is_castle: false,
    });

    test1.print_board();

    test1.process_move(&Move {
        color: Color::White,
        piece: Piece::Pawn,
        origin: Square::D5,
        destination: Square::D6,
        promote_type: None,
        is_castle: false,
    });

    test1.print_board();

    test1.process_move(&Move {
        color: Color::Black,
        piece: Piece::Knight,
        origin: Square::B8,
        destination: Square::A6,
        promote_type: None,
        is_castle: false,
    });

    test1.print_board();


    test1.process_move(&Move {
        color: Color::White,
        piece: Piece::Pawn,
        origin: Square::D6,
        destination: Square::C7,
        promote_type: None,
        is_castle: false,
    });

    test1.print_board();

    test1.process_move(&Move {
        color: Color::Black,
        piece: Piece::Knight,
        origin: Square::H6,
        destination: Square::G8,
        promote_type: None,
        is_castle: false,
    });

    test1.print_board();

    test1.process_move(&Move {
        color: Color::White,
        piece: Piece::Pawn,
        origin: Square::C7,
        destination: Square::D8,
        promote_type: Some(Piece::Bishop),
        is_castle: false,
    });
    
    test1.print_board();

    test1.process_move(&Move {
        color: Color::Black,
        piece: Piece::Knight,
        origin: Square::A6,
        destination: Square::C7,
        promote_type: Some(Piece::Bishop),
        is_castle: false,
    });

    test1.print_board();

    test1.process_move(&Move {
        color: Color::White,
        piece: Piece::Bishop,
        origin: Square::D8,
        destination: Square::C7,
        promote_type: None,
        is_castle: false,
    });

    test1.print_board();

    test1.process_move(&Move {
        color: Color::Black,
        piece: Piece::Pawn,
        origin: Square::B7,
        destination: Square::B5,
        promote_type: None,
        is_castle: false,
    });

    test1.print_board();

    test1.process_move(&Move {
        color: Color::White,
        piece: Piece::Bishop,
        origin: Square::C7,
        destination: Square::E5,
        promote_type: None,
        is_castle: false,
    });

    test1.print_board();

    test1.process_move(&Move {
        color: Color::Black,
        piece: Piece::Pawn,
        origin: Square::D7,
        destination: Square::D5,
        promote_type: None,
        is_castle: false,
    });

    test1.print_board();

    test1.process_move(&Move {
        color: Color::White,
        piece: Piece::Bishop,
        origin: Square::E5,
        destination: Square::H8,
        promote_type: None,
        is_castle: false,
    });

    test1.print_board();

    test1.process_move(&Move {
        color: Color::Black,
        piece: Piece::Bishop,
        origin: Square::F8,
        destination: Square::H6,
        promote_type: None,
        is_castle: false,
    });

    test1.print_board();

    test1.process_move(&Move {
        color: Color::White,
        piece: Piece::Pawn,
        origin: Square::E2,
        destination: Square::E3,
        promote_type: None,
        is_castle: false,
    });

    test1.print_board();

    test1.process_move(&Move {
        color: Color::Black,
        piece: Piece::King,
        origin: Square::E8,
        destination: Square::F8,
        promote_type: None,
        is_castle: false,
    });

    test1.print_board();

    test1.process_move(&Move {
        color: Color::White,
        piece: Piece::Queen,
        origin: Square::D1,
        destination: Square::G4,
        promote_type: None,
        is_castle: false,
    });

    test1.print_board();


    test1.process_move(&Move {
        color: Color::Black,
        piece: Piece::Pawn,
        origin: Square::B5,
        destination: Square::B4,
        promote_type: None,
        is_castle: false,
    });

    test1.print_board();

    test1.process_move(&Move {
        color: Color::White,
        piece: Piece::Queen,
        origin: Square::G4,
        destination: Square::D7,
        promote_type: None,
        is_castle: false,
    });

    test1.print_board();

    test1.process_move(&Move {
        color: Color::Black,
        piece: Piece::Bishop,
        origin: Square::H6,
        destination: Square::E3,
        promote_type: None,
        is_castle: false,
    });

    test1.print_board();

    test1.process_move(&Move {
        color: Color::White,
        piece: Piece::Queen,
        origin: Square::D7,
        destination: Square::D8,
        promote_type: None,
        is_castle: false,
    });

    test1.print_board();


    //println!("{:?}", Move::generate_random_move(&test1));

    println!("{:?}", RAY_ATTACKS[Direction::Southeast as usize][Square::C8 as usize].print_bitboard());

}
