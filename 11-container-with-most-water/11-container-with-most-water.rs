use std::cmp::{max, min}; 

impl Solution {
    pub fn max_area(height: Vec<i32>) -> i32 {
        let (mut lo, mut hi) = (0, height.len()-1);

        let (mut lo_max, mut hi_max) = (-1, -1);
        let mut max_area = 0;
        while lo < hi {
            let area = (hi-lo) as i32 * min(height[lo], height[hi]);
            if area > max_area {
                max_area = area;
            }

            if height[lo] < height[hi] {
                lo += 1;
                while (lo < height.len()) && (height[lo] <= lo_max) {
                    lo += 1;
                }
                lo_max = height[lo];
            } else {
                hi -= 1;
                while (hi > 0) && height[hi] <= hi_max {
                    hi -= 1;
                }
                hi_max = height[hi];
            }
        }
        max_area
    }
}