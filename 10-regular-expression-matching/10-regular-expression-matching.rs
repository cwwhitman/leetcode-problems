fn char_match<T>(mut s: T, mut p: T) -> bool where T: Clone + Iterator<Item=char> {
    while let Some(token) = p.next() {
        match token {
            'a'..='z' => {
                if s.next() != Some(token) {
                    return false;
                }
            },
            '.' => {
                if s.next().is_none() {
                    return false;
                }
            },
            '*' => {
                // They guranteed it, right?
                let repeated_char = p.next().unwrap();
                let any_char = repeated_char == '.';
                
                loop {
                    if char_match(s.clone(), p.clone()) {
                        return true;
                    }

                    if let Some(character) = s.next() {
                        if !any_char && character != repeated_char {
                            return false;
                        }
                    } else {
                        return false;
                    }
                    
                }
            },
            _ => unimplemented!("lol")
        }
    }
    
    // If we didn't consume the whole string it doesn't match
    s.next().is_none()
}

impl Solution {
    pub fn is_match(s: String, p: String) -> bool {
        char_match(s.chars().rev(), p.chars().rev())
    }
}