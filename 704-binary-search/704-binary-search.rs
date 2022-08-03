impl Solution {
    pub fn search(nums: Vec<i32>, target: i32) -> i32 {
                
        let mut upper = nums.len()-1;
        let mut lower = 0;
        
        if upper < lower {
            return -1;
        }
        
        let mut index: usize;
        loop {
            index = (upper + lower) / 2;
            let guess = nums[index];
            
            if upper <= lower {
                if guess == target {
                    return index as i32;
                } else {
                    return -1;
                }
            }
            
            if guess == target {
                return index as i32;
            }
            
            if guess > target {
                upper = index.saturating_sub(1);
            } else {
                lower = index+1;
            }
        }
    }
}