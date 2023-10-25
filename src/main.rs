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

fn main() {

    let test1 = Board::from_fen("rnbqkbnr/pppppppp/8/8/4P3/8/PPPP1PPP/RNBQKBNR w KQkq e3 0 0").unwrap();
    println!("{:?}", Move::generate_all_bishop_moves(&test1, &Square::G2));

    test1.print_board();
    println!("{:?}", Move::generate_all_queen_moves(&test1, &Square::D1));


}
