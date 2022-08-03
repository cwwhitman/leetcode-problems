impl Solution {
    pub fn first_bad_version(&self, n: i32) -> i32 {
        let mut upper = n as i64;
        let mut lower = 0;
        
        let mut guess;
        let mut last_bad = n as i64;
        loop {
            guess = (upper + lower) / 2;
            
            if self.isBadVersion(guess as i32) {
                last_bad = guess;
                upper = guess-1;
            } else {
                lower = guess+1;
            }
            
            if upper < lower {
                break;
            }
        }
        
        last_bad as i32
    }
}