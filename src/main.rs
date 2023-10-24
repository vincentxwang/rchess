#[macro_use]
extern crate lazy_static;
use crate::game::board::Board;
pub mod game;
pub mod core;
pub mod tests;
use crate::game::magic::*;

fn main() {
    println!("{:?}", NORTH_RAY_ATTACKS[62]);

    println!("{:?}", Board::from_fen("rnbqkbnr/pppppppp/8/8/4p3/8/PPPPPPPP/RNBQKBNR b KQkq e3 0 0").unwrap());
}
