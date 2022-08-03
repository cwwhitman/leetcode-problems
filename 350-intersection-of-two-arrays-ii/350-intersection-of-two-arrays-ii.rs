impl Solution {
    pub fn intersect(nums1: Vec<i32>, nums2: Vec<i32>) -> Vec<i32> {
        // sort em
        let mut n1 = nums1.clone();
        let mut n2 = nums2.clone();
        n1.sort_unstable(); n2.sort_unstable();
        
        let mut other = n2.into_iter();
        let mut last = other.next();
        n1.into_iter().filter(|&n| {
            if let Some(m) = last.take() {
                if n == m {
                    return true;
                } else if n < m {
                    last = Some(m);
                    return false;
                }
            }

            while let Some(m) = other.next() {
                if n == m {
                    return true;
                } else if n < m {
                    last = Some(m);
                    return false;
                }
            }
            false
        }).collect()
    }
}