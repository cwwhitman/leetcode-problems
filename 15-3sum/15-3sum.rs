use std::collections::{HashMap, HashSet};
use std::cmp::Ordering::{Greater, Less};

impl Solution {
    pub fn three_sum(mut nums: Vec<i32>) -> Vec<Vec<i32>> {
        nums.sort_unstable();
        let mut nums_set = HashMap::new();
        for n in &nums {
            *nums_set.entry(n).or_insert(0) += 1;
        }

        let non_negative = nums.binary_search_by(|x| x.cmp(&0).then(Greater)).unwrap_err();
        let mut results = Vec::new();
        let mut zero_count = 0;
        let mut combos: HashSet<Vec<i32>> = HashSet::new();
        for &b in nums[non_negative..].iter() {
            zero_count += if b == 0 {1} else {0};
            for &a in nums[..non_negative].iter() {
                let c = -(a+b);
                if let Some(&freq) = nums_set.get(&c) {
                    if freq >= 2 || (a != c && b != c) {
                        let (a, c) = if a > c {(c, a)} else {(a, c)};
                        let (b, c) = if b > c {(c, b)} else {(b, c)};
                        
                        if combos.insert(vec![a,b,c]) {
                            results.push(vec![a,b,c]);
                        }
                    }
                }
            }
        }
        if zero_count >= 3 {
            results.push(vec![0, 0, 0]);
        }
        results
    }
}