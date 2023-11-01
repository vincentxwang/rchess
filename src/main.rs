#[macro_use]
extern crate lazy_static;
use crate::engine::alphabeta;
use crate::engine::best_move;
use crate::game::board::Board;
pub mod game;
pub mod core;
pub mod tests;
pub mod engine;
use crate::game::movegen::moves::Move as Move;
use crate::core::structs::Square as Square;
use crate::core::structs::Color as Color;
use crate::game::piece::Piece as Piece;
use crate::engine::evaluate::Score;
use crate::engine::evaluate::material;

fn main() {


    let test1 = Board::from_fen("rnbqkbnr/pppp1ppp/4p3/8/4P3/8/PPPP1PPP/RNBQKBNR b KQkq - 0 1").unwrap();

    test1.print_board();
    println!("{:?}", alphabeta(&test1, 5, Score(-30001), Score(30001), Color::Black));
    println!("{:?}", best_move())

}
