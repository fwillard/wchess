use std::collections::HashMap;
//General utility functions

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

//Enums
pub enum Color{
    Black,
    White
}

///checks if kth bit is set in n
pub fn bit_is_set(n: &u64, k: u8) -> bool{
    let set = 0 != n & (1 << k);
    return set;
}

const fn generate_attack_table(){

}

pub fn print_bitboard(b: &u64){
    println!(" | A\u{0332}_B\u{0332}_C\u{0332}_D\u{0332}_E\u{0332}_F\u{0332}_G\u{0332}_H\u{0332} | ");

    let mut rowcount = 8;
    print!("{}|", rowcount);

    for i in (0..64).rev() {
        print!(" ");

        if bit_is_set(b, i) {
            print!("1");
        }
        else {
            print!("-");
        }

        if i % 8 == 0{
            println!(" |{}", rowcount);
            rowcount -= 1;
            if rowcount > 0{
                print!("{}|", rowcount);
            }
        }
    }

    println!(" | A\u{0305}‾B\u{0305}‾C\u{0305}‾D\u{0305}‾E\u{0305}‾F\u{0305}‾G\u{0305}‾H\u{0305} | ");
}

pub mod shift{
    use crate::util::A_FILE;
    use crate::util::H_FILE;

    //cardinal directions
    pub fn north_one(b: u64) -> u64{
        return b << 8;
    }
    pub fn south_one(b: u64) -> u64{
        return b >> 8;
    }
    pub fn east_one(b: u64) -> u64{
        return (b << 1) & !A_FILE;
    }
    pub fn west_one(b: u64) -> u64{
        return (b >> 1) & !H_FILE;
    }

    //ordinal directions
    pub fn north_east_one(b: u64) -> u64{
        return (b << 9) & !A_FILE;
    }
    pub fn south_east_one(b: u64) -> u64{
        return (b >> 7) & !A_FILE;
    }
    pub fn north_west_one(b: u64) -> u64{
        return (b << 7) & !H_FILE;
    }
    pub fn south_west_one(b: u64) -> u64{
        return (b >> 9) & !H_FILE;
    }
}
