use crate::util;
///Stores the board state as a set of bitboards
pub struct Board{
    white_pieces: Pieces,
    black_pieces: Pieces
}

struct Pieces{
    pawns: u64,
    rooks: u64,
    knights: u64,
    bishops: u64,
    queens: u64,
    king: u64
}

enum Side{
     Black,
     White
}

impl Board{
    ///Initializes a board for a standard chess game
    pub fn new() -> Board {
        Board{
            white_pieces: Pieces{
                pawns:   0x000000000000ff00,
                rooks:   0x0000000000000081,
                knights: 0x0000000000000042,
                bishops: 0x0000000000000024,
                queens:  0x0000000000000010,
                king:    0x0000000000000008,
            }, 
            
            black_pieces: Pieces{
                pawns:   0x00ff000000000000,
                rooks:   0x8100000000000000,
                knights: 0x4200000000000000,
                bishops: 0x2400000000000000,
                queens:  0x1000000000000000,
                king:    0x0800000000000000
            }
        }
    }

    ///Prints the board to the console
    pub fn print_board(&self) {
        println!(" | A\u{0332}_B\u{0332}_C\u{0332}_D\u{0332}_E\u{0332}_F\u{0332}_G\u{0332}_H\u{0332} | ");
        let mut rowcount = 8;
        print!("{}|", rowcount);
        for i in (0..64).rev() {
            print!(" ");
            if util::bit_is_set(self.white_pieces.pawns, i) {
                print!("p");
            }
            else if util::bit_is_set(self.white_pieces.rooks, i){
                print!("r");
            }
            else if util::bit_is_set(self.white_pieces.knights, i){
                print!("n");
            }
            else if util::bit_is_set(self.white_pieces.bishops, i){
                print!("b");
            }
            else if util::bit_is_set(self.white_pieces.queens, i){
                print!("q");
            }
            else if util::bit_is_set(self.white_pieces.king, i){
                print!("k");
            }
            else if util::bit_is_set(self.black_pieces.pawns, i) {
                print!("P");
            }
            else if util::bit_is_set(self.black_pieces.rooks, i){
                print!("R");
            }
            else if util::bit_is_set(self.black_pieces.knights, i){
                print!("N");
            }
            else if util::bit_is_set(self.black_pieces.bishops, i){
                print!("B");
            }
            else if util::bit_is_set(self.black_pieces.queens, i){
                print!("Q");
            }
            else if util::bit_is_set(self.black_pieces.king, i){
                print!("K");
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
    fn gen_moves(&self, s: Side){

    }
    // ///Returns a bitboard with the location of all the white pieces
    // fn white_pieces(&self) -> u64 {
    //     return self.white_pawns | self.white_rooks | self.white_knights | self.white_bishops | self.white_queen | self.white_king;
    // }
    // ///Returns a bitboard with the location of all the black pieces
    // fn black_pieces(&self) -> u64 {
    //     return self.black_pawns | self.black_rooks | self.black_knights | self.black_bishops | self.black_queen | self.black_king;
    // }
    // ///Returns a bitboard with the location of all pieces
    // fn all_pieces(&self) -> u64{
    //     return self.white_pieces() | self.black_pieces();
    // }
}