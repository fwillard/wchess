use std::collections::HashMap;

use crate::board::Board;
use crate::util;
use crate::util::shift;
use crate::util::Color;

struct Move {
    from: u8,
    to: u8,
    move_type: MoveType,
}

enum MoveType {
    Quiet,
    Capture,
    EnPassant,
    Castle,
}

#[derive(PartialEq, Eq, Hash)]
enum PieceType {
    Pawn,
    Rook,
    Knight,
    Bishop,
    Queen,
    King,
}
lazy_static! {
    static ref KNIGHT_ATTACKS: [u64; 64] = generate_knight_attacks();
}
pub fn generate_moves() {}

fn generate_knight_attacks() -> [u64; 64] {
    let mut attack_table = [0; 64];
    for i in 0..64 {
        let pos = 1 << i;
        let attacked = knight_attacks(pos);
        attack_table[i] = attacked;
        println!();
    }
    return attack_table;
}

fn knight_attacks(pos: u64) -> u64 {
    let mut east = shift::east_one(pos);
    let mut west = shift::west_one(pos);
    let mut attacks = (east | west) << 16;
    attacks |= (east | west) >> 16;

    // util::print_bitboard(&attacks);

    east = shift::east_one(east);
    west = shift::west_one(west);
    attacks |= (east | west) << 8;
    attacks |= (east | west) >> 8;

    return attacks;
}

fn pawn_push_targets(pos: Board, side: Color) -> u64 {
    let empty = !pos.all_pieces();

    match side {
        Color::White => {
            let single_push = shift::north_one(pos.white_pieces.pawns) & empty;
            let double_push = shift::north_one(single_push) & empty & util::RANK_4;

            return single_push | double_push;
        }
        Color::Black => {
            let single_push = shift::south_one(pos.black_pieces.pawns) & empty;
            let double_push = shift::south_one(single_push) & empty & util::RANK_5;

            return single_push | double_push;
        }
    }
}
