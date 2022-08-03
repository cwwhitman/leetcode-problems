impl Solution {
    pub fn get_row(row_index: i32) -> Vec<i32> {
        let row_index = row_index as usize;

        let mut row =  Vec::with_capacity(row_index+1);
        row.push(1);

        for i in 0..row_index {
            let mut last = 0;
            for n in row.iter_mut() {
                let temp = *n;
                *n += last;
                last = temp;
            }
            row.push(1);
        }

        row
    }
}