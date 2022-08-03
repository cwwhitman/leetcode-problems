use std::cmp::Ordering::Less;

fn min(a: Option<&i32>, b: Option<&i32>) -> i32 {
    std::cmp::min(*a.unwrap_or(&i32::MAX), *b.unwrap_or(&i32::MAX))
}

impl Solution {
    pub fn find_median_sorted_arrays(nums1: Vec<i32>, nums2: Vec<i32>) -> f64 {
        let total_len = nums1.len() + nums2.len();
        let halfway = (total_len - 1)/2;
        let odd = total_len % 2 == 1;

        let (mut i, mut j) = (0, 0);

        let (mut new_i, mut new_j);
        (loop {
            //println!("i: {}, j: {}", i, j);
            if nums2.len() > 0 {
                let first2 = &nums2[j];
                new_i = i + nums1[i..]
                    .binary_search_by(|x| x.cmp(first2).then(Less)).unwrap_err();
                if new_i + j > halfway {
                    if odd {
                        break nums1[halfway - j];
                    } else {
                        // println!("wrong1");
                        break nums1[halfway - j] +
                            min(nums1.get(halfway - j + 1), nums2.get(j));//wrong
                    }
                } else if new_i == nums1.len() {
                    if odd {
                        break nums2[halfway - nums1.len()];
                    } else {
                        break nums2[halfway - nums1.len()] + nums2[halfway - nums1.len() + 1];
                    }
                }
                i = new_i;
            }


            let first1 = &nums1[i];
            new_j = j + nums2[j..]
                .binary_search_by(|x| x.cmp(first1).then(Less)).unwrap_err();
            if i + new_j > halfway {
                if odd {
                    break nums2[halfway - i];
                } else {
                    //println!("wrong2");
                    break nums2[halfway - i] +
                        min(nums2.get(halfway - i + 1), nums1.get(i));
                    //break nums2[halfway - i] + nums2[halfway - i + 1];//wrong
                }
            } else if new_j == nums2.len() {
                if odd {
                    break nums1[halfway - nums2.len()];
                } else {
                    break nums1[halfway - nums2.len()] + nums1[halfway - nums2.len() + 1];
                }

            }
            j = new_j;
        }) as f64 / (if odd {1.0} else {2.0})
    }
}