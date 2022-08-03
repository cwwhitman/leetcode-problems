use std::iter;

impl Solution {
    pub fn generate(num_rows: i32) -> Vec<Vec<i32>> {
        let num_rows = num_rows as usize;

        let mut triangle =  Vec::with_capacity(num_rows);
        triangle.push(vec![1]);
        for i in 1..num_rows {
            triangle.push(triangle.last().unwrap().iter().scan(0, |last, n| {
                let &mut temp = last;
                *last = *n;
                Some(temp + n)
            }).chain(iter::once(1)).collect());
        }

        triangle
    }
}