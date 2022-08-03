use std::cmp::max;

impl Solution {
    pub fn rob(nums: Vec<i32>) -> i32 {
        if nums.len() <= 1 {
            return nums[0];
        }
        
        let mut houses_with = [0, 0, 0, nums[0]];
        let mut houses_without = [0; 4];
        for &house in &nums[1..] {
            houses_with[0] = house;
            houses_with.rotate_left(1);
            houses_with[3] += max(houses_with[0], houses_with[1]);

            houses_without[0] = house;
            houses_without.rotate_left(1);
            houses_without[3] += max(houses_without[0], houses_without[1]);
        }
        //println!("{:?}\n{:?}", houses_with, houses_without);
        max(
            max(houses_with[1], houses_with[2]),
            houses_without[3]
        )
    }
}