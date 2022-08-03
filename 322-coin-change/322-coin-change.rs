use std::collections::{HashSet, VecDeque};

impl Solution {
    pub fn coin_change(mut coins: Vec<i32>, amount: i32) -> i32 {
        coins.sort_unstable();
        let mut queue = VecDeque::from([(amount, 0)]);
        let mut reached = [false; 10001];
        
        while let Some((amount, count)) = queue.pop_front() {
            if reached[amount as usize] {
                continue;
            } else {
                reached[amount as usize] = true;
            }
            
            if amount == 0 {
                return count;
            }

            for &coin in &coins {
                if coin > amount { break; }
                queue.push_back((amount - coin, count+1));
            }
        }
        -1
    }
}