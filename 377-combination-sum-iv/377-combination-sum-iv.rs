impl Solution {
    pub fn combination_sum4(mut nums: Vec<i32>, target: i32) -> i32 {
        nums.sort_unstable();

        let mut saved = [[0; 1001]; 201];
        for t in 1..=target {
            for l in 1..=nums.len() {
                let last_num = nums[l-1];
                saved[l][t as usize] = if last_num > t {
                    saved[l-1][t as usize]
                } else {
                    (if last_num == t {1} else {0}) +
                    nums[..l].iter()
                    .map(|x| saved[l][(t-x) as usize])
                    .sum::<i32>()
                }
            }
        }
        saved[nums.len()][target as usize]
    }
}