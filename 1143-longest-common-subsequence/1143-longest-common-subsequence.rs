fn lcs(text1: &[u8], text2: &[u8], saved: &mut [[i32; 1001]; 1001]) -> i32 {
    let result = saved[text1.len()][text2.len()];
    if result != -1 {
        return result;
    }

    let mut max = 0;
    let mut earliest = text2.len();
    for (i, ch1) in text1.iter().enumerate() {
        for (j, ch2) in text2[..earliest].iter().enumerate() {
            if ch1 == ch2 {
                max = max.max(1 + lcs(&text1[i+1..], &text2[j+1..], saved));
                earliest = j;
                break;
            }
        }
    }
    
    saved[text1.len()][text2.len()] = max;
    max
}

impl Solution {
    pub fn longest_common_subsequence(text1: String, text2: String) -> i32 {
        let mut saved = [[-1; 1001]; 1001];
        lcs(text1.as_bytes(), text2.as_bytes(), &mut saved)
    }
}