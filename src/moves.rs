use crate::bitutil;
use crate::board::Board;
use crate::util::Color;

pub struct Move {
    from: u8,
    to: u8,
    move_type: MoveType,
}

struct Magic {
    mask: u64,
    magic: u64,
    attacks: u64,
}

enum MoveType {
    Quiet,
    Capture,
    EnPassant,
    Castle,
}

//sliding pieces
// fn generate_rook_magic() {
//     for i in 0..64 {
//         let m = Magic{
//             mask:
//         }
//     }
// }

//pawn moves & attacks
fn pawn_push_targets(pos: Board, side: Color) -> u64 {
    let empty = !pos.all_pieces();

    match side {
        Color::White => {
            let single_push = bitutil::north_n(pos.white_pieces.pawns, 1) & empty;
            let double_push = bitutil::north_n(single_push, 1) & empty & bitutil::RANK_4;

            return single_push | double_push;
        }
        Color::Black => {
            let single_push = bitutil::south_n(pos.black_pieces.pawns, 1) & empty;
            let double_push = bitutil::south_n(single_push, 1) & empty & bitutil::RANK_5;

            return single_push | double_push;
        }
    }
}
