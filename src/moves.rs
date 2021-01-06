use crate::board::Board;
use crate::util::Color;
use crate::util::shift;

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
            let rank_4: u64 = 0x00000000ff000000;
            let single_push = shift::north_one(pos.white_pieces.pawns) & empty;
            let double_push = shift::north_one(single_push) & empty & rank_4;

            return single_push | double_push;
        }
        Color::Black => {
            let rank_5: u64 = 0x000000ff00000000;
            let single_push = shift::south_one(pos.black_pieces.pawns) & empty;
            let double_push = shift::south_one(single_push) & empty & rank_5;

            return single_push | double_push;
        }
    }
}