mod board;
mod util;


fn main() {
    
    let b = board::Board::new();

    b.print_board();

    // util::bit_is_set(0xffff, 0);

}
