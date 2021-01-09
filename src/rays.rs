//! Module containing directional ray generation functions
use self::Direction::*;
use crate::bitutil;
use enum_map::{enum_map, Enum, EnumMap};
use std::slice::Iter;

lazy_static! {
    /// Internal directional bitboard map
    static ref RAYS: EnumMap<Direction, [u64; 64]> = init();
}

///Enum representing the cardinal and ordinal directions
#[derive(Debug, Enum)]
pub enum Direction {
    North,
    East,
    South,
    West,
    NorthEast,
    NorthWest,
    SouthEast,
    SouthWest,
}

impl Direction {
    ///iterates through the direction enum
    pub fn iterator() -> Iter<'static, Direction> {
        static DIRECTIONS: [Direction; 8] = [
            North, South, East, West, NorthEast, NorthWest, SouthEast, SouthWest,
        ];
        DIRECTIONS.iter()
    }
}

///Initialize the ray maps. Called lazily at startup by lazy static intialization
fn init() -> EnumMap<Direction, [u64; 64]> {
    let mut map = enum_map! {
        North => {
            let mut b: [u64; 64] = [0; 64];
            for sq in 0..64 {
                b[sq] = 0x0101010101010100 << sq;
            }
            return b;
        },
        East => {
            let mut b: [u64; 64] = [0; 64];
            for sq in 0..64 {
                b[sq] = (1 << sq) - (1 << (sq & 56));
            }
            return b;
        }
        South => {
            let mut b: [u64; 64] = [0; 64];
            for sq in 0..64 {
                b[sq] = 0x0080808080808080 >> (63 - sq);
            }
            return b;
        },
        West => {
            let mut b: [u64; 64] = [0; 64];
            for sq in 0..64 {
                b[sq] = 2 * ((1 << (sq | 7)) - (1 << sq));
            }
            return b;
        },
        NorthEast => {
            let mut b: [u64; 64] = [0; 64];
            let mut noea = 0x102040810204000;
            for f in 0..8 {//iterate through files
                let mut ne = noea;
                for r in 0..8{ // iterate through ranks
                    // let idx = r*8 + f;
                    let idx = bitutil::index(r, f);
                    b[idx] = ne;
                    ne = ne << 8;
                }
                noea = bitutil::east_n(noea, 1);
             }
             return b;
        },
        NorthWest => {
            let mut b: [u64; 64] = [0; 64];
            let mut nowe = 0x8040201008040200;
            for f in (0..8).rev() {//iterate through files
                let mut nw = nowe;
                for r in 0..8{ // iterate through ranks
                    // let idx = r*8 + f;
                    let idx = bitutil::index(r, f);
                    b[idx] = nw;
                    nw = nw << 8;
                }
                nowe = bitutil::west_n(nowe, 1);
             }
             return b;
        },
        SouthEast => {
            let mut b: [u64; 64] = [0; 64];
            let mut soea = 0x40201008040201;
            for f in 0..8 {//iterate through files
                let mut se = soea;
                for r in (0..8).rev(){ // iterate through ranks
                    // let idx = r*8 + f;
                    let idx = bitutil::index(r, f);
                    b[idx] = se;
                    se = se >> 8;
                }
                soea = bitutil::east_n(soea, 1);
            }
            return b;
        },
        SouthWest => {
            let mut b: [u64; 64] = [0; 64];
            let mut sowe = 0x2040810204080;
            for f in (0..8).rev() {//iterate through files
                let mut sw = sowe;
                for r in (0..8).rev(){ // iterate through ranks
                    // let idx = r*8 + f;
                    let idx = bitutil::index(r, f);
                    b[idx] = sw;
                    sw = sw >> 8;
                }
                sowe = bitutil::west_n(sowe, 1);
             }
             return b;
        },
    };

    return map;
}

pub fn get_ray(dir: Direction, sq: usize) -> u64 {
    return RAYS[dir][sq];
}
