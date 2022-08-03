impl Solution {
    pub fn can_jump(nums: Vec<i32>) -> bool {
        let mut farthest = 0;
        for (&max_jump, lily) in nums.iter().zip(0..nums.len() as i32) {
            if lily > farthest {
                return false;
            }
            farthest = farthest.max(lily+max_jump);
        }
        true
    }
}