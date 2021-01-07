use crate::util;
use regex::Captures;
use regex::Regex;
use std::fmt;
///Stores the board state as a set of bitboards
#[derive(Copy, Clone)]
pub struct Board {
    pub white_pieces: Pieces,
    pub black_pieces: Pieces,
    pub side_to_move: util::Color,
    pub castle_rights: [bool; 4],
    pub en_passant: Option<u64>,
    pub half_move_clock: u8,
    pub full_move_number: u64,
}

#[derive(Copy, Clone)]
pub struct Pieces {
    pub pawns: u64,
    pub rooks: u64,
    pub knights: u64,
    pub bishops: u64,
    pub queens: u64,
    pub king: u64,
}

impl Board {
    ///Initializes a board for a standard chess game
    pub fn new() -> Board {
        //initialize board with default FEN string
        return Board::from("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1");
    }
    ///Initializes a board from a FEN string
    pub fn from(fen: &str) -> Board {
        let tokens: Vec<&str> = fen.split(" ").collect();

        let side = match tokens[1] {
            "w" => util::Color::White,
            "b" => util::Color::Black,
            _ => panic!(),
        };

        let mut castle = [false, false, false, false];

        if tokens[2].contains("K") {
            castle[0] = true;
        }
        if tokens[2].contains("Q") {
            castle[1] = true;
        }
        if tokens[2].contains("k") {
            castle[2] = true;
        }
        if tokens[2].contains("q") {
            castle[3] = true;
        }

        let ep = match tokens[3] {
            "-" => None,
            _ => Some(util::algebraic_to_index(tokens[3])),
        };

        let half = tokens[4].parse::<u8>().unwrap();
        let full = tokens[5].parse::<u64>().unwrap();

        //generate board layout
        let mut w_p: u64 = 0x0000000000000000;
        let mut w_r: u64 = 0x0000000000000000;
        let mut w_n: u64 = 0x0000000000000000;
        let mut w_b: u64 = 0x0000000000000000;
        let mut w_q: u64 = 0x0000000000000000;
        let mut w_k: u64 = 0x0000000000000000;

        let mut b_p: u64 = 0x0000000000000000;
        let mut b_r: u64 = 0x0000000000000000;
        let mut b_n: u64 = 0x0000000000000000;
        let mut b_b: u64 = 0x0000000000000000;
        let mut b_q: u64 = 0x0000000000000000;
        let mut b_k: u64 = 0x0000000000000000;

        //remove the row separation
        let stripped = tokens[0].replace("/", "");

        //replaces all numbers with the corresponding number of empty spaces (represented by ".")
        //for example "7", gets replaced with "......."
        let re = Regex::new(r"[0-9]").unwrap();
        let stripped = re.replace_all(&stripped, |caps: &Captures| -> String {
            return ".".repeat(caps[0].parse::<usize>().unwrap());
        });

        //the resulting string should be the same length as the board
        assert_eq!(stripped.len(), 64);

        //iterate through all squares of the string, and set the corresponding bit in the correct pieces bitboard
        for (i, c) in stripped.chars().rev().enumerate() {
            match c {
                'p' => b_p = util::set_bit(b_p, i),
                'r' => b_r = util::set_bit(b_r, i),
                'n' => b_n = util::set_bit(b_n, i),
                'b' => b_b = util::set_bit(b_b, i),
                'q' => b_q = util::set_bit(b_q, i),
                'k' => b_k = util::set_bit(b_k, i),

                'P' => w_p = util::set_bit(w_p, i),
                'R' => w_r = util::set_bit(w_r, i),
                'N' => w_n = util::set_bit(w_n, i),
                'B' => w_b = util::set_bit(w_b, i),
                'Q' => w_q = util::set_bit(w_q, i),
                'K' => w_k = util::set_bit(w_k, i),

                '.' => (),
                _ => panic!(),
            }
        }

        Board {
            white_pieces: Pieces {
                pawns: w_p,
                rooks: w_r,
                knights: w_n,
                bishops: w_b,
                queens: w_q,
                king: w_k,
            },

            black_pieces: Pieces {
                pawns: b_p,
                rooks: b_r,
                knights: b_n,
                bishops: b_b,
                queens: b_q,
                king: b_k,
            },
            side_to_move: side,
            castle_rights: castle,
            en_passant: ep,
            half_move_clock: half,
            full_move_number: full,
        }
    }
    ///Returns a bitboard with the location of all the white pieces
    pub fn white_pieces(&self) -> u64 {
        return self.white_pieces.pawns
            | self.white_pieces.rooks
            | self.white_pieces.knights
            | self.white_pieces.bishops
            | self.white_pieces.queens
            | self.white_pieces.king;
    }
    ///Returns a bitboard with the location of all the black pieces
    pub fn black_pieces(&self) -> u64 {
        return self.black_pieces.pawns
            | self.black_pieces.rooks
            | self.black_pieces.knights
            | self.black_pieces.bishops
            | self.black_pieces.queens
            | self.black_pieces.king;
    }
    ///Returns a bitboard with the location of all pieces
    pub fn all_pieces(&self) -> u64 {
        return self.white_pieces() | self.black_pieces();
    }
}

impl fmt::Debug for Board {
    // This trait requires `fmt` with this exact signature.
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // Write strictly the first element into the supplied output
        // stream: `f`. Returns `fmt::Result` which indicates whether the
        // operation succeeded or failed. Note that `write!` uses syntax which
        // is very similar to `println!`.
        writeln!(
            f,
            " | A\u{0332}_B\u{0332}_C\u{0332}_D\u{0332}_E\u{0332}_F\u{0332}_G\u{0332}_H\u{0332} | "
        )
        .unwrap();
        let mut rowcount = 8;
        write!(f, "{:?}|", rowcount).unwrap();
        for i in (0..64).rev() {
            write!(f, " ").unwrap();
            if util::bit_is_set(&self.white_pieces.pawns, i) {
                write!(f, "p").unwrap();
            } else if util::bit_is_set(&self.white_pieces.rooks, i) {
                write!(f, "r").unwrap();
            } else if util::bit_is_set(&self.white_pieces.knights, i) {
                write!(f, "n").unwrap();
            } else if util::bit_is_set(&self.white_pieces.bishops, i) {
                write!(f, "b").unwrap();
            } else if util::bit_is_set(&self.white_pieces.queens, i) {
                write!(f, "q").unwrap();
            } else if util::bit_is_set(&self.white_pieces.king, i) {
                write!(f, "k").unwrap();
            } else if util::bit_is_set(&self.black_pieces.pawns, i) {
                write!(f, "P").unwrap();
            } else if util::bit_is_set(&self.black_pieces.rooks, i) {
                write!(f, "R").unwrap();
            } else if util::bit_is_set(&self.black_pieces.knights, i) {
                write!(f, "N").unwrap();
            } else if util::bit_is_set(&self.black_pieces.bishops, i) {
                write!(f, "B").unwrap();
            } else if util::bit_is_set(&self.black_pieces.queens, i) {
                write!(f, "Q").unwrap();
            } else if util::bit_is_set(&self.black_pieces.king, i) {
                write!(f, "K").unwrap();
            } else {
                write!(f, "-").unwrap();
            }
            if i % 8 == 0 {
                write!(f, " |{:?}", rowcount).unwrap();
                rowcount -= 1;
                let ep_string = match self.en_passant {
                    None => String::from("-"),
                    Some(i) => util::index_to_algebraic(i),
                };
                match rowcount {
                    7 => writeln!(f, "       Side to move: {:?}", self.side_to_move).unwrap(),
                    6 => writeln!(f, "    Castling Rights: {:?}", self.castle_rights).unwrap(),
                    5 => writeln!(f, "         En Passant: {}", ep_string).unwrap(),
                    4 => writeln!(f, "    Half-Move Clock: {:?}", self.half_move_clock).unwrap(),
                    3 => writeln!(f, "   Full-Move Number: {:?}", self.full_move_number).unwrap(),
                    2 => writeln!(f, "                FEN: ").unwrap(),
                    _ => writeln!(f).unwrap(),
                }

                if rowcount > 0 {
                    write!(f, "{:?}|", rowcount).unwrap();
                }
            }
        }

        writeln!(
            f,
            " | A\u{0305}‾B\u{0305}‾C\u{0305}‾D\u{0305}‾E\u{0305}‾F\u{0305}‾G\u{0305}‾H\u{0305} | "
        )
        .unwrap();

        fmt::Result::Ok(())
    }
}
