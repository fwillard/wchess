///Stores the board state as a set of bitboards
pub struct Board{
    white_pawns: u64,
    white_rooks: u64,
    white_knights: u64,
    white_bishops: u64,
    white_queen: u64,
    white_king: u64,

    black_pawns: u64,
    black_rooks: u64,
    black_knights: u64,
    black_bishops: u64,
    black_queen: u64,
    black_king: u64,
}


impl Board{
    ///Initializes a board for a standard chess game
    pub fn new() -> Board {
        Board{
            white_pawns:   0x000000000000ff00,
            white_rooks:   0x0000000000000081,
            white_knights: 0x0000000000000042,
            white_bishops: 0x0000000000000024,
            white_queen:   0x0000000000000010,
            white_king:    0x0000000000000008,
            
            black_pawns:   0x00ff000000000000,
            black_rooks:   0x8100000000000000,
            black_knights: 0x4200000000000000,
            black_bishops: 0x2400000000000000,
            black_queen:   0x1000000000000000,
            black_king:    0x0800000000000000
        }
    }

    ///Prints the board to the console
    fn print_board(&self) {
        
    }

    ///Returns a bitboard with the location of all the white pieces
    fn white_pieces(&self) -> u64 {
        return self.white_pawns | self.white_rooks | self.white_knights | self.white_bishops | self.white_queen | self.white_king;
    }
    ///Returns a bitboard with the location of all the black pieces
    fn black_pieces(&self) -> u64 {
        return self.black_pawns | self.black_rooks | self.black_knights | self.black_bishops | self.black_queen | self.black_king;
    }
    ///Returns a bitboard with the location of all pieces
    fn all_pieces(&self) -> u64{
        return self.white_pieces() | self.black_pieces();
    }
}