#[macro_use]
extern crate lazy_static;
use crate::engine::alphabeta;
use crate::engine::best_move;
use crate::game::board::Board;
use std::io;
use std::string;
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

    static PLAYER_IS_WHITE: bool = false;

    static ENGINE_COLOR: Color = Color::White;

    let mut game = Board::new();

    while Score::get_score(&game) != Score(30000) && Score::get_score(&game) != Score(-30000) {
        if !PLAYER_IS_WHITE && game.meta.full_moves == 1 {
            game.print_board();
            // alphabeta(&game, 4, Score(-30001), Score(30001), Color::White);
            let play = best_move(&game, 4);
            println!("engine plays: {:?}", play);
            game.process_move(&play);
        }
        game.print_board();
        println!("play a move!");
        let mut player_move = String::new();
        io::stdin().read_line(&mut player_move).unwrap();
        let player_move = Move::from_uci(&game, &player_move);
        game.process_move(&player_move);
        game.print_board();

        //alphabeta(&game, 4, Score(-30001), Score(30001), ENGINE_COLOR);
        let play = best_move(&game, 4);
        println!("engine plays: {:?}", play);
        game.process_move(&play);
    }

}
