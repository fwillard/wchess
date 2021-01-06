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
    pub static ref KNIGHT_ATTACKS: [u64; 64] = generate_knight_attacks();
    pub static ref KING_ATTACKS: [u64; 64] = generate_king_attacks();
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

fn generate_king_attacks() -> [u64; 64] {
    let mut attack_table = [0; 64];
    for i in 0..64 {
        let pos = 1 << i;
        let attacked = king_attacks(pos);
        attack_table[i] = attacked;
        println!();
    }
    return attack_table;
}

fn king_attacks(pos: u64) -> u64 {
    let north = shift::north_one(pos);
    let south = shift::south_one(pos);
    let east = shift::east_one(pos);
    let west = shift::west_one(pos);

    let north_east = shift::north_east_one(pos);
    let north_west = shift::north_west_one(pos);
    let south_east = shift::south_east_one(pos);
    let south_west = shift::south_west_one(pos);

    return north | south | east | west | north_east | north_west | south_east | south_west;
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
