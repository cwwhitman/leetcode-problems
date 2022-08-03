const GOLDEN_RATIO: f64 = 1.618033988749894;
const SQRT_FIVE: f64 = 2.23606797749979;

impl Solution {
    pub fn climb_stairs(n: i32) -> i32 {
        (GOLDEN_RATIO.powi(n+1)/SQRT_FIVE).round() as i32
    }
}