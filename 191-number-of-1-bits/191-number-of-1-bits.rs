impl Solution {
    pub fn hammingWeight (mut n: u32) -> i32 {
        let mut weight = 0;
        while n != 0 {
            weight += n & 1;
            n >>= 1;
        }
        weight as i32
    }
}