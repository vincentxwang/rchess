use crate::{game::{board::Board, movegen::moves::Move}, 
    core::structs::Color};
use crate::engine::evaluate::Score;
use crate::engine::zobrist::*;

pub mod evaluate;
pub mod zobrist;

pub fn alphabeta(node: &Board, depth: usize, mut alpha: Score, mut beta: Score, player: Color) -> Score {
    let all_moves = Move::generate_legal_moves(node);

    if all_moves.is_empty() {
        if node.meta.player == Color::White {
            return Score(-30000);
        } else {
            return Score(30000);
        }
    }

    if depth == 0 {
        return Score::get_score(node);
    } 

    if TRANSPOSITION_TABLE.lock().unwrap().contains_key(&node.meta.zobrist) {
        return TRANSPOSITION_TABLE.lock().unwrap()[&node.meta.zobrist];
    }
    
    // if player is maximizing!
    if player == Color::White {
        let mut max_eval = Score(-30001);
        for move_candidate in all_moves {
            let mut new_board = *node;
            new_board.process_move(&move_candidate);
            let eval = alphabeta(&new_board, depth - 1, alpha, beta, Color::Black);
            max_eval = std::cmp::max(max_eval, eval);
            alpha = std::cmp::max(eval, alpha);
            
            if beta <= alpha {
                break;
            }
        }
        max_eval
    } else {
        let mut min_eval = Score(30001);
        for move_candidate in all_moves {
            let mut new_board = *node;
            new_board.process_move(&move_candidate);
            let eval = alphabeta(&new_board, depth - 1, alpha, beta, Color::White);
            min_eval = std::cmp::min(min_eval, eval);
            beta = std::cmp::min(beta, eval);
            if beta <= alpha {
                break;
            }
        }
        min_eval
    }
}

pub fn root_alphabeta(board: &Board, depth: usize) -> (Option<Move>, Score) {

    let mut current_best_eval: (Option<Move>, Score) = match board.meta.player {
        Color::White => (None, Score(-30001)),
        Color::Black => (None, Score(30001)),
    };

    for candidate_move in Move::generate_legal_moves(board) {
        let mut new_board = *board;
        new_board.process_move(&candidate_move);
        let new_eval = alphabeta(
            &new_board,
            depth - 1,
            Score(-30001),
            Score(30001),
            Color::not(board.meta.player));

        TRANSPOSITION_TABLE.lock().unwrap().insert(new_board.meta.zobrist, new_eval);

        match board.meta.player {
            Color::White => {
                if new_eval >= current_best_eval.1 {
                    current_best_eval = (Some(candidate_move), new_eval);
                }
            },
            Color::Black => {
                if new_eval <= current_best_eval.1 {
                    current_best_eval = (Some(candidate_move), new_eval);
                }
            }
        } 
    }
    current_best_eval
}