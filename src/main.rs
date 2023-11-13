#[macro_use]
extern crate lazy_static;

use crate::engine::root_alphabeta;
use crate::engine::zobrist::Zobrist;
use crate::game::board::Board;
use std::io;

pub mod game;
pub mod core;
pub mod tests;
pub mod engine;
use crate::game::movegen::moves::Move as Move;
use crate::core::structs::Color as Color;
use crate::engine::evaluate::Score;

fn read_i32() -> i32 {
    let line = io::stdin().lines().next().unwrap().unwrap();
    line.parse().unwrap()
}

fn main() {
    let game = Board::new();
    println!("{:?}", Zobrist::zobrist_hash(&game));
    
    println!("pick a player color! (B: 1 or W: 0)");
 
    println!("Input desired color");
    let number1 = read_i32() as usize;

    let engine_color = match number1 {
        1 => Color::White,
        0 => Color::Black,
        _ => panic!("pick 'B' or 'W'!"),
    };

    println!("Input desired depth");
    let number1 = read_i32() as usize;

    // Board initialization.
    let mut game = Board::new();

    // Engine plays first, if White.
    if engine_color == Color::White && game.meta.full_moves == 1 {
        game.print_board();
        let play = root_alphabeta(&game, number1);
        println!("engine plays: {:?}", play);
        game.process_move(&play);
        game.print_board();
    }

    while Score::get_score(&game) != Score(30000) && Score::get_score(&game) != Score(-30000) {
        println!("play a move!");
        let mut player_move = String::new();
        io::stdin().read_line(&mut player_move).unwrap();
        let player_move = Move::from_uci(&game, &player_move);
        game.process_move(&player_move);
        game.print_board();
        println!("{:?}", game.meta.zobrist);

        let play = root_alphabeta(&game, number1);
        println!("engine plays: {:?}", play);
        game.process_move(&play);
        game.print_board();
        println!("{:?}", game.meta.zobrist);
    }
    

}
