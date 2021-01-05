///General utility functions

///checks if kth bit is set in n
pub fn bit_is_set(n: u64, k: u8) -> bool{
    let set = 0 != n & (1 << k);
    return set;
}