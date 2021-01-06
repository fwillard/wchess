use crate::board::Board;
use crate::util::Color;
use crate::util::shift;
use crate::util;

struct Move{
    from: u8,
    to: u8,
    move_type: MoveType 
}

enum MoveType{
    Quiet,
    Capture,
    EnPassant,
    Castle
}

// pub fn gen_moves(pos: Board) -> Vec<Move> {

// }

pub fn pawn_push_targets(pos: Board, side: Color) -> u64 {

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