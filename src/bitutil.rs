//! Bitboard constants and utility functions
//bitboard constants
// Empty set
pub const EMPTY: u64 = 0;

// Universe set
pub const UNIVERSAL: u64 = 0xffffffffffffffff;

//Chessboard Files
pub const A_FILE: u64 = 0x8080808080808080;
pub const B_FILE: u64 = 0x4040404040404040;
pub const C_FILE: u64 = 0x2020202020202020;
pub const D_FILE: u64 = 0x1010101010101010;
pub const E_FILE: u64 = 0x0808080808080808;
pub const F_FILE: u64 = 0x0404040404040404;
pub const G_FILE: u64 = 0x0202020202020202;
pub const H_FILE: u64 = 0x0101010101010101;

//Chessboard ranks
pub const RANK_1: u64 = 0x00000000000000ff;
pub const RANK_2: u64 = 0x000000000000ff00;
pub const RANK_3: u64 = 0x0000000000ff0000;
pub const RANK_4: u64 = 0x00000000ff000000;
pub const RANK_5: u64 = 0x000000ff00000000;
pub const RANK_6: u64 = 0x0000ff0000000000;
pub const RANK_7: u64 = 0x00ff000000000000;
pub const RANK_8: u64 = 0xff00000000000000;

// //set lsb to 0 and return index
// pub fn pop_lsb(&b: u64) -> u64 {}

///Moves all bits n squares north
pub fn north_n(b: u64, n: usize) -> u64 {
    let mut new_board = b;
    for _ in 0..n {
        new_board = new_board << 8;
    }
    return new_board;
}
///Moves all bits n squares south
pub fn south_n(b: u64, n: usize) -> u64 {
    let mut new_board = b;
    for _ in 0..n {
        new_board = new_board >> 8;
    }
    return new_board;
}
///Moves all bits n squares east
pub fn east_n(b: u64, n: usize) -> u64 {
    let mut new_board = b;
    for _ in 0..n {
        new_board = (new_board >> 1) & !A_FILE;
    }
    return new_board;
}
///Moves all bits n squares west
pub fn west_n(b: u64, n: usize) -> u64 {
    let mut new_board = b;
    for _ in 0..n {
        new_board = (new_board << 1) & !H_FILE;
    }
    return new_board;
}
///Moves all bits n squares north east
pub fn north_east_n(b: u64, n: usize) -> u64 {
    let mut new_board = b;
    for _ in 0..n {
        new_board = (new_board << 7) & !A_FILE;
    }
    return new_board;
}
///Moves all bits n squares south east
pub fn south_east_n(b: u64, n: usize) -> u64 {
    let mut new_board = b;
    for _ in 0..n {
        new_board = (new_board >> 9) & !A_FILE;
    }
    return new_board;
}
///Moves all bits n squares north west
pub fn north_west_n(b: u64, n: usize) -> u64 {
    let mut new_board = b;
    for _ in 0..n {
        new_board = (new_board << 9) & !H_FILE;
    }
    return new_board;
}
///Moves all bits n squares south west
pub fn south_west_n(b: u64, n: usize) -> u64 {
    let mut new_board = b;
    for _ in 0..n {
        new_board = (new_board >> 7) & !H_FILE;
    }
    return new_board;
}

///Returns the column for a given square
pub fn col(sq: usize) -> usize {
    return 7 - sq % 8;
}
///Returns the row for a given square
pub fn row(sq: usize) -> usize {
    return sq / 8;
}

///returns the square index for a given row and file
pub fn index(r: usize, f: usize) -> usize {
    return r * 8 + (7 - f);
}
