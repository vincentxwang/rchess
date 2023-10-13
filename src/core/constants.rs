// Numerical constants -----------
// Number of players on a chessboard.
pub const PLAYER_COUNT: usize = 2;
// Piece types on the chessboard.
pub const PIECETYPE_COUNT: usize = 6;



// Initial bitboards --------------
// All white pieces starting bitboard
pub const WHITE_START: u64 = 0b_00000000_00000000_00000000_00000000_00000000_00000000_11111111_11111111;
// All black pieces starting bitboard
pub const BLACK_START: u64 = 0b_11111111_11111111_00000000_00000000_00000000_00000000_00000000_00000000;
// All pawns starting bitboard
pub const PAWN_START: u64 = 0b_00000000_11111111_00000000_00000000_00000000_00000000_11111111_00000000;
// All bishops starting bitboard
pub const BISHOP_START: u64 = 0b_00100100_00000000_00000000_00000000_00000000_00000000_00000000_00100100;
// All knights starting bitboard
pub const KNIGHT_START: u64 = 0b_01000010_00000000_00000000_00000000_00000000_00000000_00000000_01000010;
// All rooks starting bitboard
pub const ROOK_START: u64 = 0b_10000001_00000000_00000000_00000000_00000000_00000000_00000000_10000001;
// All queens starting bitboard
pub const QUEEN_START: u64 = 0b_00001000_00000000_00000000_00000000_00000000_00000000_00000000_00001000;
// All kings starting bitboard
pub const KING_START: u64 = 0b_00010000_00000000_00000000_00000000_00000000_00000000_00000000_00010000;