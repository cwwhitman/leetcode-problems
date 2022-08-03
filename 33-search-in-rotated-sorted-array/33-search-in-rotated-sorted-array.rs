impl Solution {
    pub fn search(nums: Vec<i32>, target: i32) -> i32 {
        let first = nums.first().unwrap();
        let index = nums.binary_search_by(|x| {
            if (target >= *first) ^ (x < first) {
                x.cmp(&target)
            } else {
                x.cmp(&target).reverse()
            }
        });

        if let Ok(i) = index {
            i as i32
        } else {
            -1
        }
    }
}