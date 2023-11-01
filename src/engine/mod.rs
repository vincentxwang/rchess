use std::sync::Mutex;
use crate::{game::{board::Board, movegen::moves::Move}, 
    core::structs::Color};
use crate::engine::evaluate::Score;

pub mod evaluate;

lazy_static! {
    static ref bestMove: Mutex<Vec<Move>> = Mutex::new(vec![]);
}

pub fn alphabeta(node: &Board, depth: usize, mut alpha: Score, mut beta: Score, player: Color) -> Score {
    if depth == 0 {
        return Score::get_score(node);
    } 
    if Move::generate_legal_moves(node).is_empty() {
        if node.meta.player == Color::White {
            return Score(30000);
        } else {
            return Score(-30000);
        }
    }
    if node.meta.player == player {
        let mut max_eval = Score(-30001);
        for move_candidate in Move::generate_legal_moves(node) {
            let mut new_board = node.clone();
            new_board.process_move(&move_candidate);
            let eval = alphabeta(&new_board, depth - 1, alpha, beta, player);
            max_eval = std::cmp::max(max_eval, eval);
            if eval >= alpha {
                alpha = eval;
                bestMove.lock().unwrap().pop();
                bestMove.lock().unwrap().push(move_candidate);
            }
            
            if beta <= alpha {
                break;
            }
        }
        return max_eval;
    } else {
        let mut min_eval = Score(30001);
        for move_candidate in Move::generate_legal_moves(node) {
            let mut new_board = node.clone();
            new_board.process_move(&move_candidate);
            let eval = alphabeta(&new_board, depth - 1, alpha, beta, player);
            min_eval = std::cmp::min(min_eval, eval);
            beta = std::cmp::min(beta, eval);
            if beta <= alpha {
                break;
            }
        }
        return min_eval;       
    }
}

pub fn best_move() -> Move {
    return bestMove.lock().unwrap()[0];
}