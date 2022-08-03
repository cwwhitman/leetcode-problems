impl Solution {
    pub fn matrix_reshape(mat: Vec<Vec<i32>>, r: i32, c: i32) -> Vec<Vec<i32>> {
        let (r, c) = (r as usize, c as usize);

        if mat.len() * mat[0].len() != r * c 
        || mat.len() == r && mat[0].len() == c {
            return mat;
        }
        
        let mut new_mat = Vec::with_capacity(r);
        new_mat.push(Vec::with_capacity(c));
        
        for row in mat.into_iter() {
            for element in row.into_iter() {
                let last_row = new_mat.last_mut().unwrap();
                if last_row.len() < c {
                    last_row.push(element)
                } else {
                    let mut new_row = Vec::with_capacity(c);
                    new_row.push(element);
                    new_mat.push(new_row)
                }
            }
        }

        new_mat
    }
}