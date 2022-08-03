impl Solution {
    pub fn num_decodings(s: String) -> i32 {
        let s = s.as_bytes();
        let mut counts = [0, 0, 1];
        if let Some(&last) = s.last() {
            counts.rotate_left(1);
            counts[2] = if last == b'0' {
                0
            } else { 1 };
        }
        
        for i in (0..s.len()-1).rev() {
            counts.rotate_left(1);
            counts[2] = match s[i] {
                b'0' => {
                    0
                },
                b'1' => {
                    counts[1] + counts[0]
                },
                b'2' => {
                    counts[1] + if (b'0'..=b'6').contains(&s[i+1]) {
                        counts[0]
                    } else { 0 }
                },
                _ => {
                    counts[1]
                },
            };
        }
        counts[2]
    }
}