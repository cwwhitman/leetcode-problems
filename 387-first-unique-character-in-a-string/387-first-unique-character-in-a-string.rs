#[derive(Copy, Clone)]
enum Letter {
    NotFound,
    First(isize),
    Repeats,
}

impl Solution {
    pub fn first_uniq_char(s: String) -> i32 {
        let mut letters = [Letter::NotFound; 26];
        
        for (i, c) in s.chars().enumerate() {
            let letter = c as usize - 97;
            letters[letter] = match letters[letter] {
                Letter::NotFound => {
                    Letter::First(i as isize)
                },
                _ => {
                    Letter::Repeats
                }
            }
        }
        
        letters.into_iter().filter_map(|&c| {
            if let Letter::First(i) = c {
                Some(i)
            } else {
                None
            }
        }).min().unwrap_or(-1) as i32
    }
}