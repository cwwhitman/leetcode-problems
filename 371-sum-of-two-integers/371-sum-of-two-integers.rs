impl Solution {
    pub fn get_sum(mut a: i32, b: i32) -> i32 {
        let mut carry = b;
        let mut new_carry;
        while carry != 0 {
            new_carry = (a & carry) << 1;
            a ^= carry;
            carry = new_carry;
        }
        a
    }
}