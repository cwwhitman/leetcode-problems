impl Solution {
    pub fn max_sub_array(nums: Vec<i32>) -> i32 {
        let mut sum = nums[0];
        let mut max = nums[0];
        for &n in nums.iter().skip(1) {
            if sum < 0 {
                sum = 0;
            }
            sum += n;
            max = if sum > max { sum } else { max };
        }
        max
    }
}