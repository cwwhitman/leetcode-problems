struct TrieNode {
    indexes: Vec<i32>,
    follows: [Option<Box<TrieNode>>; 26],
}

impl TrieNode {
    pub fn new() -> Self {
        TrieNode {
            indexes: vec![],
            follows: Default::default(),
        }
    }
    
    fn contains<T>(&self, mut word: T) -> &[i32]
        where T: Iterator<Item = char>
    {
        if let Some(c) = word.next() {
            if let Some(next_c) = &self.follows[(c as u8 - 'a' as u8) as usize] {
                next_c.contains(word)
            } else {
                &self.indexes[..0]
            }
        } else {
            &self.indexes[..]
        }
    }
    
    fn insert<T>(&mut self, mut word: T, index: i32) 
        where T: Iterator<Item = char>
    {
        self.indexes.push(index);
        
        if let Some(c) = word.next() {
            self.follows[(c as u8 - 'a' as u8) as usize]
                .get_or_insert(Box::new(TrieNode::new()))
                .insert(word, index);
        }
    }
}

struct WordFilter {
    starts: TrieNode,
    ends: TrieNode,
}


/** 
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl WordFilter {

    fn new(words: Vec<String>) -> Self {
        let mut dict = WordFilter {
            starts: TrieNode::new(),
            ends: TrieNode::new(),
        };
        
        for (i, word) in words.iter().enumerate() {
            dict.starts.insert(word.chars(), i as i32);
            dict.ends.insert(word.chars().rev(), i as i32);
        }
        
        dict
    }
    
    fn f(&self, prefix: String, suffix: String) -> i32 {
        let pre_words = self.starts.contains(prefix.chars());
        let suf_words = self.ends.contains(suffix.chars().rev());
        
       // println!("{:?}\n{:?}", pre_words, suf_words);
        
        let mut pre_words = pre_words.iter().rev();
        let mut suf_words = suf_words.iter().rev();
        
        let mut index = i32::MAX;
        
        loop {
            loop {
                if let Some(&pre_idx) = pre_words.next() {
                    if pre_idx < index {
                        index = pre_idx;
                        break;
                    } else if pre_idx == index {
                        return index;
                    }
                } else {
                    return -1;
                }
            }
            
            loop {
                if let Some(&suf_idx) = suf_words.next() {
                    if suf_idx < index {
                        index = suf_idx;
                        break;
                    } else if suf_idx == index {
                        return index;
                    }
                } else {
                    return -1;
                }
            }
        }
    }
}