use std::cmp::max;

impl Solution {
    pub fn rob(nums: Vec<i32>) -> i32 {
        let mut houses = [0; 4];
        for house in nums {
            houses.rotate_left(1);
            houses[3] = house;
            houses[3] += max(houses[0], houses[1]);
        }
        max(houses[2], houses[3])
    }
}