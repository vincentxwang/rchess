use crate::{game::{board::Board, movegen::moves::Move}, 
    core::structs::Color};
use crate::engine::evaluate::Score;
use crate::engine::zobrist::*;

pub mod evaluate;
pub mod zobrist;

// Recursively performs an alpha-beta prune.
// alpha -> best (maximum) value white can guarantee
// beta -> best (minimum) value black can guarantee
pub fn alphabeta(node: &Board, depth: usize, mut alpha: Score, mut beta: Score, player: Color) -> Score {
    let all_moves = Move::generate_legal_moves(node);

    // Check if is "checkmate" (Draws not implemented :( )
    if all_moves.is_empty() {
        return if node.meta.player == Color::White {Score(-30000)} else {Score(30000)};
    }

    if depth == 0 {
        return Score::get_score(node);
    } 

    if TRANSPOSITION_TABLE.lock().unwrap().contains_key(&node.meta.zobrist) {
        return TRANSPOSITION_TABLE.lock().unwrap()[&node.meta.zobrist];
    }
    
    // White seeks to maximize the evaluation, while black seeks to minimize it.
    if player == Color::White {
        // Holds maximum evaluation.
        let mut eval = Score(-30001);
        for move_candidate in all_moves {

            // Creates new board. Pretty inefficient. TODO: write undo method
            let mut new_board = *node;
            new_board.process_move(&move_candidate);
            eval = std::cmp::max(
                eval,
                alphabeta(&new_board, depth - 1, alpha, beta, Color::Black));

            // alpha = max score white can guarantee from this position
            alpha = std::cmp::max(eval, alpha);
            
            // "prunes" the tree (stops search), because black would never choose this!
            if beta <= eval {
                break;
            }
        }
        eval
    } else {
        let mut eval = Score(30001);
        for move_candidate in all_moves {
            let mut new_board = *node;
            new_board.process_move(&move_candidate);
            eval = std::cmp::min(
                eval,
                alphabeta(&new_board, depth - 1, alpha, beta, Color::White));
            beta = std::cmp::min(beta, eval);
            if eval <= alpha {
                break;
            }
        }
        eval
    }
}

pub fn root_alphabeta(board: &Board, depth: usize) -> Move {
    let mut current_best_move: Option<Move> = None;

    let mut current_best_eval = match board.meta.player {
        Color::White => Score(-30001),
        Color::Black => Score(30001),
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

        if board.meta.player == Color::White && 
            new_eval >= current_best_eval {
                    current_best_move = Some(candidate_move);
                    current_best_eval = new_eval;
        }
        if board.meta.player == Color::Black && 
            new_eval <= current_best_eval {
                    current_best_move = Some(candidate_move);
                    current_best_eval = new_eval;
        }     
    }
    println!("{:?}", current_best_eval);
    current_best_move.unwrap()
}