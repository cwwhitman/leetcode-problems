use std::vec;

impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        for (i, &n) in nums.iter().enumerate() {
            for (j, &m) in nums[i+1..].iter().enumerate() {
                if n + m == target {
                    return vec![i as i32, (i+j+1) as i32];
                }
            }
        }
        vec![-1, -1]
    }
}