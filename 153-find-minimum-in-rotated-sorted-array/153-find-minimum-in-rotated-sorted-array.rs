use std::cmp::Ordering::Less;


impl Solution {
    pub fn find_min(nums: Vec<i32>) -> i32 {
        let not_min = nums.first().unwrap();
        nums[
            nums.binary_search_by(|x| x.cmp(not_min).reverse().then(Less)).unwrap_err()
            % nums.len()
        ]
    }
}