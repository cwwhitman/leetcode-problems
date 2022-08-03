impl Solution {
    pub fn is_anagram(s: String, t: String) -> bool {
        if s.len() != t.len() {
            return false;
        }
        
        let mut letters = [0i32; 26];
        for (x, y) in s.chars().zip(t.chars()) {
            letters[x as usize - 'a' as usize] += 1;
            letters[y as usize - 'a' as usize] -= 1;
        }
        
        letters == [0i32; 26]
    }
}