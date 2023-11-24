// zobrist.rs provides Zobrist hashing and maintains a global transposition table.

use rand::Rng;
use std::collections::HashMap;
use std::fs::File;
use std::io::{Write, BufReader, BufRead};
use std::sync::Mutex;

use crate::engine::evaluate::Score;
use crate::{game::board::Board, core::structs::{Color, Square}};

lazy_static! {
    // ZOBRIST_TABLE contains pseudorandom numbers for every piece type + piece color on every square.
    pub static ref ZOBRIST_TABLE: [[[u64; 64]; 6]; 2] = Zobrist::get_zobrist_constants().unwrap();
    pub static ref BLACK_TO_MOVE: [u64; 1] = [13023143897365832559];
    pub static ref TRANSPOSITION_TABLE: Mutex<HashMap<Zobrist, Score>> = Mutex::new(HashMap::new());
}

#[derive(Debug, Copy, Clone, Eq, Hash, PartialEq)]
pub struct Zobrist(pub u64);

impl Zobrist {
    // Writes new constants to zobrist_constants.txt (comment out as needed)
    pub fn write_zobrist_constants() -> std::io::Result<()> {

        let file = "zobrist_constants.txt";
        let mut output = File::create(file)?;
        for _i in 0..2 {
            for _j in 0..6 {
                for _k in 0..64 {
                    writeln!(output, "{}", get_random_u64());
                }
            }
        }
        Ok(())
    }


    // Reads constants from zobrist_constants.txt
    fn get_zobrist_constants() -> Result<[[[u64; 64]; 6]; 2], std::io::Error> {

        // Paths are from the src in Rust!
        let file = File::open("src/engine/zobrist_constants.txt")?;

        let br = BufReader::new(file);
        let mut table =  [[[0; 64]; 6]; 2];

        let mut lines = br.lines();
        
        for i in 0..2 {
            for j in 0..6 {
                for k in 0..64 {
                    table[i][j][k] = lines.next().unwrap().unwrap().parse::<u64>().unwrap();
                }
            }
        }
        Ok(table)
    }

    // Finds the zobrist_hash of a board. This should only be used once in a game because it is easier to find the zobrist_hash
    // by XOR'ing during the move.
    pub fn zobrist_hash(board: &Board) -> Zobrist {
        let mut hash: u64 = 0;
        if board.meta.player == Color::Black {
            // For some reason lazy_static requires this to be an array?
            hash ^= BLACK_TO_MOVE[0];
        }
        for sq in 0..64 {
            if let Some(i) = board.get_piece(&Square::from_int(sq)) {
                hash ^= ZOBRIST_TABLE[i.1 as usize][i.0 as usize][sq];
            }
        }
        Zobrist(hash)
    }

}

fn get_random_u64() -> u64 {
    let mut rng = rand::thread_rng();
    rng.gen::<u64>()
}