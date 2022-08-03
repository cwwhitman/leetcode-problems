impl Solution {
    pub fn reverse_bits(mut x: u32) -> u32 {
        let mut rev = 0u32;
        for _ in 0..32 {
            rev <<= 1;
            rev |= x & 1;
            x >>= 1;
        }
        rev
    }
}
