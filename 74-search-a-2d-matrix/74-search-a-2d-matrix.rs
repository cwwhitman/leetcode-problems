use std::cmp::Ordering;

impl Solution {
    pub fn search_matrix(matrix: Vec<Vec<i32>>, target: i32) -> bool {
        let maybe_row = matrix.binary_search_by(|row| {
            if *row.first().unwrap() > target {
                Ordering::Greater
            } else if *row.last().unwrap() < target {
                Ordering::Less
            } else {
                Ordering::Equal
            }
        });
        
        if let Ok(row) = maybe_row {
            matrix[row].binary_search(&target).is_ok()
        } else {
            false
        }
    }
}