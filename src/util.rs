///General utility functions

///checks if kth bit is set in n
pub fn bit_is_set(n: &u64, k: u8) -> bool{
    let set = 0 != n & (1 << k);
    return set;
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

pub enum Color{
    Black,
    White
}

pub mod shift{
    const NOT_A_FILE: u64 = 0x7f7f7f7f7f7f7f7f;
    const NOT_H_FILE: u64 = 0xfefefefefefefefe;

    //cardinal directions
    pub fn north_one(b: u64) -> u64{
        return b << 8;
    }
    pub fn south_one(b: u64) -> u64{
        return b >> 8;
    }
    pub fn east_one(b: u64) -> u64{
        return (b << 1) & NOT_A_FILE;
    }
    pub fn west_one(b: u64) -> u64{
        return (b >> 1) & NOT_H_FILE;
    }

    //ordinal directions
    pub fn north_east_one(b: u64) -> u64{
        return (b << 9) & NOT_A_FILE;
    }
    pub fn south_east_one(b: u64) -> u64{
        return (b >> 7) & NOT_A_FILE;
    }
    pub fn north_west_one(b: u64) -> u64{
        return (b << 7) & NOT_H_FILE;
    }
    pub fn south_west_one(b: u64) -> u64{
        return (b >> 9) & NOT_H_FILE;
    }
}
