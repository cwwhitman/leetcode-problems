use std::collections::HashMap;

fn wb<'a>(s: &'a str, words: &mut HashMap<&'a str, bool>) -> bool {
    if let Some(&is_word) = words.get(s) {
        return is_word;
    }

    let mut result = false;
    for i in 0..=s.len() {
        if let Some(&prefix_is_word) = words.get(&s[0..i]) {
            if wb(&s[i..], words) && prefix_is_word {
                result = true;
                break;
            }
        }
    }

    words.insert(s, result);
    result
}

impl Solution {
    pub fn word_break(s: String, word_dict: Vec<String>) -> bool {
        let mut words = word_dict.iter().map(|x| (x.as_str(), true)).collect();
        wb(s.as_str(), &mut words)
    }
}