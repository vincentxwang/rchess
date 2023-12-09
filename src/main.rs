//

#![feature(test)]
#[macro_use]
extern crate lazy_static;

use crate::engine::root_alphabeta;
use crate::game::board::Board;
use std::io;

pub mod game;
pub mod core;
pub mod tests;
pub mod engine;
use crate::game::movegen::moves::Move as Move;
use crate::core::structs::Color as Color;
use crate::engine::evaluate::Score;

fn main() {

    // User inputs engine (and player) color.
    let engine_color = get_engine_color();

    // User inputs depth.
    let depth = get_depth();

    // Board initialization.
    let mut game = Board::new();
    game.print_board();

    // Engine plays first, if White and on the first move.
    if engine_color == Color::White && game.meta.full_moves == 1 {
        process_engine_turn(&mut game, depth);
    }

    while Score::get_score(&game) != Score(30000) && Score::get_score(&game) != Score(-30000) {
        process_player_turn(&mut game, depth);
        process_engine_turn(&mut game, depth);
    }

}

fn process_player_turn(game: &mut Board, depth: usize) {
    println!("Play a move using long algebraic notation (e.g. 1.e4 is e2e4)!");
    let mut player_move = String::new();
    io::stdin().read_line(&mut player_move).unwrap();
    let player_move = Move::from_uci(&game, &player_move);
    game.process_move(&player_move);
    game.print_board();
}

fn process_engine_turn(game: &mut Board, depth: usize) {
    let play = root_alphabeta(&game, depth);
    println!("engine plays: {:?}", play);
    game.process_move(&play);
    game.print_board();
}

fn get_engine_color() -> Color {
    println!("Pick a player color! Type '0' for White and '1' for Black.");
    let mut user_input = String::new();
    let stdin = io::stdin();

    loop {
        stdin.read_line(&mut user_input).expect("Error when reading line in get_engine_color()");
        match user_input.trim().parse::<i32>() {
            Ok(0) => return Color::Black,
            Ok(1) => return Color::White,
            Ok(_val) => {
                println!("Please input either 0 or 1!");
                user_input.clear();
            },
            Err(e) => {
                println!("Please input a valid number! Error: {}", e);
                user_input.clear();
            }
        }
    }
}

fn get_depth() -> usize {
    println!("Pick the desired depth of the engine. Currently, 5 is probably the best choice. Large numbers will fail to feasibly run.");
    let mut user_input = String::new();
    let stdin = io::stdin();

    loop {
        stdin.read_line(&mut user_input).expect("Error when reading line in get_depth()");
        match user_input.trim().parse::<i32>() {
            Ok(val) => {
                if val > 0 {
                    return val as usize;
                } 
                println!("Enter a positve number");
                user_input.clear();
            },
            Err(e) => {
                println!("Please input a valid number! Error: {}", e);
                user_input.clear();
            }       
        }
    }
}


