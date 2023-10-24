#[macro_use]
extern crate lazy_static;
use crate::game::board::Board;
pub mod game;
pub mod core;
pub mod tests;
use crate::game::magic::*;
use crate::core::structs::Direction as Direction;

fn main() {
    println!("{:?}", RAY_ATTACKS[Direction::North as usize][62]);

    println!("{:?}", Board::from_fen("rnbqkbnr/pppppppp/8/8/4p3/8/PPPPPPPP/RNBQKBNR b KQkq e3 0 0").unwrap());
}
