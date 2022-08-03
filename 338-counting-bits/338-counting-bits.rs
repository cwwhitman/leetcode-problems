impl Solution {
    pub fn count_bits(n: i32) -> Vec<i32> {
        let mut set_bits = vec![0; (n+1) as usize];
        for i in 1..set_bits.len() {
            set_bits[i] = set_bits[i-1] - (!(i - 1)).trailing_zeros() as i32 + 1;
        }
        set_bits
    }
}