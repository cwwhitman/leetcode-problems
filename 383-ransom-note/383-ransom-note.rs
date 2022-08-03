impl Solution {
    pub fn can_construct(ransom_note: String, magazine: String) -> bool {
        let mut magazine_letters = [0u32; 26];
        for c in magazine.chars() {
            magazine_letters[c as usize - 'a' as usize] += 1;
        }
        
        for c in ransom_note.chars() {
            let remaining = magazine_letters[c as usize - 'a' as usize].checked_sub(1);
            if let Some(r) = remaining {
                magazine_letters[c as usize - 'a' as usize] = r;
            } else {
                return false;
            }
        }
        true
    }
}