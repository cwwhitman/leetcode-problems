use std::cmp::max;

impl Solution {
    pub fn max_profit(prices: Vec<i32>) -> i32 {
        let mut prices = prices.into_iter();
        prices.fold((i32::MAX, 0), |(low, profit), price| {
            if price <= low {
                (price, profit)
            } else {
                (low, max(profit, price - low))
            }
        }).1
    }
}