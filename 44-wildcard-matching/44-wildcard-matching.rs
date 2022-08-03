fn pattern_match<T>(mut s: T, mut p: T) -> (bool, bool) where T: Clone + Iterator<Item=char> {
    while let Some(token) = p.next() {
        match token {
            'a'..='z' => {
                if s.next() != Some(token) {
                    return (false, false);
                }
            },
            '?' => {
                if s.next().is_none() {
                    return (false, false);
                }
            },
            '*' => {
                loop {
                    let check = pattern_match(s.clone(), p.clone());
                    if check.0 {
                        return (true, true);
                    } else if check.1 {
                        return (false, true)
                    }

                    if s.next().is_none() {
                        return (false, true);
                    }
                }
            },
            _ => unimplemented!("lol")
        }
    }
    
    // If we didn't consume the whole string it doesn't match
    (s.next().is_none(), false)
}

impl Solution {
    pub fn is_match(s: String, p: String) -> bool {
        pattern_match(s.chars(), p.chars()).0
    }
}