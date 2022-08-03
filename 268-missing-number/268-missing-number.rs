impl Solution {
    pub fn missing_number(nums: Vec<i32>) -> i32 {
        let mut sum = 0;
        for (n, i) in nums.iter().zip(1..=(nums.len() as i32)){
            sum ^= n ^ i;
        }
        sum
    }
}