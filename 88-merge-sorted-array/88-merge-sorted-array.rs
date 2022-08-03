impl Solution {
    pub fn merge(nums1: &mut Vec<i32>, m: i32, nums2: &mut Vec<i32>, n: i32) {
        let mut nums1_clone = nums1.clone();
        
        let mut i = 0;
        let mut n1_index = 0;
        let mut n2_index = 0;

        loop {
            if n1_index == m as usize {
                nums1[i..].copy_from_slice(&nums2[n2_index..(n as usize)]);
                break;
            }
            if n2_index == n as usize {
                nums1[i..].copy_from_slice(&nums1_clone[n1_index..(m as usize)]);
                break;
            }

            if nums1_clone[n1_index] < nums2[n2_index] {
                nums1[i] = nums1_clone[n1_index];
                n1_index += 1;
            } else {
                nums1[i] = nums2[n2_index];
                n2_index += 1;
            }
            i += 1;
        }
    }
}