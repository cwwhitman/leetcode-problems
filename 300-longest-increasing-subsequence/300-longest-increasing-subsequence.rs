use std::cmp::Ordering;

impl Solution {
    pub fn length_of_lis(nums: Vec<i32>) -> i32 {
        let mut subseq = Vec::new();
        for num in nums {
            let num_pos = subseq.binary_search(&num);
            if let Err(i) = num_pos {
                if i == subseq.len() {
                    subseq.push(num);
                } else {
                    subseq[i] = num;
                }
            }
        }
        subseq.len() as i32
    }
}