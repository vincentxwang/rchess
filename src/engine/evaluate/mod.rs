pub mod material;
pub mod pst;

use std::ops::Add;
use std::ops::Sub;
use crate::core::structs::Color;
use crate::game::board::Board;


#[derive(Debug)]
pub struct Score(i16);

impl Score {

    // get_score gets the score of the entire position.
    pub fn get_score(board: &Board) -> Score {
        Score::get_side_score(board, Color::White) - Score::get_side_score(board, Color::Black)
    }

    fn get_side_score(board: &Board, color: Color) -> Score {
        let mut score = Score(0);

        for sq in board.sides[color as usize].get_squares() {
            score = score + material::piece_eval(board.get_piece(&sq).unwrap().0);   
            score = score + pst::get_pst_eval(color, &sq, board.get_piece(&sq).unwrap().0);
        }

        score
    }
}

impl Add for Score {
    type Output = Self;

    fn add(self, other: Score) -> Score {
        Score(self.0 + other.0)
    }
}

impl Sub for Score {
    type Output = Self;

    fn sub(self, other: Score) -> Score {
        Score(self.0 - other.0)
    }
}