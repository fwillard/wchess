//! General utility functions
use regex::Regex;

//Enums
#[derive(Copy, Clone, Debug)]
pub enum Color {
    Black,
    White,
}

///checks if kth bit is set in n
pub fn bit_is_set(n: &u64, k: usize) -> bool {
    let set = 0 != n & (1 << k);
    return set;
}

pub const fn set_bit(n: u64, k: usize) -> u64 {
    return n | 1 << k;
}

pub const fn unset_bit(n: u64, k: usize) -> u64 {
    return n & !(1 << k);
}

pub fn algebraic_to_index(s: &str) -> u64 {
    let re = Regex::new(r"(?P<file>[a-h])(?P<rank>[1-8])").unwrap();

    let caps = re.captures(s).unwrap();
    let file = caps.name("file").unwrap().as_str();
    let rank = caps.name("rank").unwrap().as_str();

    let file = file.to_uppercase().chars().next().unwrap() as u64 - 64;
    let rank = rank.parse::<u64>().unwrap();
    return 8 * (rank - 1) + (8 - file);
}

pub fn index_to_algebraic(n: u64) -> String {
    let rank = (n / 8 + 1) as u8;
    let file = (7 - (n % 8)) as u8;
    let mut s = String::from("");

    s.push((file + 0x61) as char);
    s.push((rank + 0x30) as char);
    return s;
}

pub fn get_square_index(rank: usize, file: usize) -> usize {
    return 8 * rank + (8 - file);
}

pub fn print_bitboard(b: &u64) {
    println!(
        " | A\u{0332}_B\u{0332}_C\u{0332}_D\u{0332}_E\u{0332}_F\u{0332}_G\u{0332}_H\u{0332} | "
    );

    let mut rowcount = 8;
    print!("{}|", rowcount);

    for i in (0..64).rev() {
        print!(" ");

        if bit_is_set(b, i) {
            print!("1");
        } else {
            print!("-");
        }

        if i % 8 == 0 {
            println!(" |{}", rowcount);
            rowcount -= 1;
            if rowcount > 0 {
                print!("{}|", rowcount);
            }
        }
    }

    println!(
        " | A\u{0305}‾B\u{0305}‾C\u{0305}‾D\u{0305}‾E\u{0305}‾F\u{0305}‾G\u{0305}‾H\u{0305} | "
    );
}

// pub mod shift {
//     use ::bitutil::A_FILE;
//     use ::bitutil::H_FILE;

//     pub fn rank_mask(sq: usize) -> u64 {
//         return 0xff << (sq & 56);
//     }
//     pub fn file_mask(sq: usize) -> u64 {
//         return 0x0101010101010101 << (sq & 7);
//     }
//     pub fn diag_mask(sq: usize) -> u64 {
//         const MAIN_DIAG: u64 = 0x8040201008040201;
//         let sq = sq as isize;
//         let diag = 8 * (sq & 7) - (sq & 56);
//         let north = -diag & (diag >> 31);
//         let south = diag & (-diag >> 31);
//         return (MAIN_DIAG >> south) << north;
//     }
//     pub fn anti_diag_mask(sq: usize) -> u64 {
//         const MAIN_DIAG: u64 = 0x0102040810204080;
//         let sq = sq as isize;
//         let diag = 56 - 8 * (sq & 7) - (sq & 56);
//         let nort = -diag & (diag >> 31);
//         let sout = diag & (-diag >> 31);
//         return (MAIN_DIAG >> sout) << nort;
//     }
// }
