use std::collections::BTreeMap;

struct SummaryRanges {
    intervals: BTreeMap<i32, i32>,
}


/** 
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl SummaryRanges {

    fn new() -> Self {
        SummaryRanges {
            intervals: BTreeMap::new(),
        }
    }
    
    fn add_num(&mut self, val: i32) {
        let mut intervals = self.intervals.range_mut(..=val+1);
        
        let mut add = Some((val, val));
        let mut remove = None;
        while let Some((start, end)) = intervals.next() {
            if (*start..=*end).contains(&val) {
                return;
            }
            
            if val + 1 == *start {
                add = Some((val, *end));
                remove = Some(*start);
                break;
            }
            
            if val == *end + 1 {
                add = None;
                *end = val;
                if let Some((next_start, next_end)) = intervals.next() {
                    if val + 1 == *next_start {
                        *end = *next_end;
                        remove = Some(*next_start);
                        break;
                    }
                }
            }
        }
        
        if let Some(key) = remove {
            self.intervals.remove(&key);
        }
        
        if let Some(entry) = add {
            self.intervals.insert(entry.0, entry.1);
        }
    }
    
    fn get_intervals(&self) -> Vec<Vec<i32>> {
        self.intervals.iter().map(|(&s, &e)| vec![s, e]).collect()
    }
}