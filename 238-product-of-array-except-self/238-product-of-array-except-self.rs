impl Solution {
    pub fn product_except_self(nums: Vec<i32>) -> Vec<i32> {
        let mut zeroes = 0u8;
        let mut zero_location = 0;

        let mut product = 1;
        for (i, &n) in nums.iter().enumerate() {
            if n != 0 {
                product *= n;
            } else {
                zeroes += 1;
                if zeroes == 2 {
                    break;
                }
                zero_location = i;
            }
        }

        let mut answer;
        if zeroes == 0 {
            answer = vec![nums.iter().product(); nums.len()];
            for (a, &n) in answer.iter_mut().zip(nums.iter()) {
                *a /= n;
            }
        } else if zeroes == 1 {
            answer = vec![0; nums.len()];
            answer[zero_location] = product;
        } else {
            answer = vec![0; nums.len()]
        }
        answer
    }
}