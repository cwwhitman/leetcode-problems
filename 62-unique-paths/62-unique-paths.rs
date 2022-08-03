impl Solution {
    pub fn unique_paths(m: i32, n: i32) -> i32 {
        let (m, n) = (m as usize, n as usize);
        let (small, large) = if m < n {
            (m, n)
        } else {
            (n, m)
        };
        
        let mut row = vec![0; small];
        row[0] = 1;
        
        for r in 0..large {
            for c in 1..small {
                row[c] += row[c-1];
            }
        }
        
        row[small-1]
    }
}