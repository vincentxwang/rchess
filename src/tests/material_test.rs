#[cfg(test)]
mod tests {

    use crate::game::bitboard::Bitboard as Bitboard;
    use crate::core::structs::Square as Square;
    use crate::engine::evaluate::material::mvv_lva;
    use crate::game::board::Board;
    use crate::game::movegen::moves::Move as Move;

    // Bitboard tests
    #[test]
    fn test_mvv_lva() {
        let kiwipete = Board::from_fen("r3k2r/p1ppqpb1/bn2pnp1/3PN3/1p2P3/2N2Q1p/PPPBBPPP/R3K2R w KQkq - 0 1").unwrap();
        // White queen takes black pawn is expected to be worse than white queen takes black knight
        assert!(mvv_lva(&Move::from_uci(&kiwipete, "f3h3"), &kiwipete) < mvv_lva(&Move::from_uci(&kiwipete, "f3f6"), &kiwipete));
        // White pawn takes black pawn is expected to be better than white knight takes black pawn
        assert!(mvv_lva(&Move::from_uci(&kiwipete, "d5e6"), &kiwipete) > mvv_lva(&Move::from_uci(&kiwipete, "e5d7"), &kiwipete));
    }
}