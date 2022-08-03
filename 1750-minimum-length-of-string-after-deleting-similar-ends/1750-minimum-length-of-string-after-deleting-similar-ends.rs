impl Solution {
    pub fn minimum_length(s: String) -> i32 {
        let s = s.as_bytes();
        let (mut lo, mut hi) = (0, s.len()-1);
        
        while s[lo] == s[hi] {
            if lo == hi {
                return 1;
            }
            let character = s[lo];
            while s[lo] == character {
                lo += 1;
                if lo == hi {
                    return 0;
                }
            }
            while s[hi] == character {
                hi -= 1;
            }
        }
        (hi - lo + 1) as i32
    }
}