lazy_static! {
    static ref KNIGHT_ATTACKS: [u64; 64] = init_knight_attacks();
    static ref KING_ATTACKS: [u64; 64] = init_king_attacks();

    static ref ROOK_MASK: [u64; 64] = init_rook_mask();
    static ref BISHOP_MASK: [u64; 64] = init_bishop_mask();

    // static ref ROOK_MAGICS: [Magic; 64];
    // static ref BISHOP_MAGICS: [Magic; 64];
}

//rooks
fn init_rook_mask() -> [u64; 64] {
    let mut attack_table = [0; 64];
    for i in 0..64 {
        let attacked = rook_masks(i);
        attack_table[i] = attacked;
    }
    return attack_table;
}
fn rook_masks(sq: usize) -> u64 {
    let file = shift::file_mask(sq) & 0x00ffffffffffff00; //mask out ranks 1 and 8
    let rank = shift::rank_mask(sq) & 0x7e7e7e7e7e7e7e7e; //mask out files A & H

    let combined = rank | file;

    let attacks = util::unset_bit(combined, sq);
    return attacks;
}

// bishops
fn init_bishop_mask() -> [u64; 64] {
    let mut attack_table = [0; 64];
    for i in 0..64 {
        let attacked = bishop_masks(i);
        attack_table[i] = attacked;
    }
    return attack_table;
}
fn bishop_masks(sq: usize) -> u64 {
    let combined = (shift::diag_mask(sq) | shift::anti_diag_mask(sq)) & 0x007e7e7e7e7e7e00; //mask out ranks 1 & 8 and files a & h
    let attacks = util::unset_bit(combined, sq);
    return attacks;
}

fn bishop_attacks(sq: usize, blockers: u64) {}

// //queen
// fn init_queen_mask() -> [u64; 64] {
//     let mut attack_table = [0; 64];
//     for i in 0..64 {
//         let attacked = ROOK_MASK[i] | BISHOP_MASK[i];
//         util::print_bitboard(&attacked);
//         attack_table[i] = attacked;
//     }
//     return attack_table;
// }
//knight attacks
fn init_knight_attacks() -> [u64; 64] {
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

//king attacks
fn init_king_attacks() -> [u64; 64] {
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
