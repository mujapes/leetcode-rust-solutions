impl Solution {
    pub fn matrix_reshape(mat: Vec<Vec<i32>>, r: i32, c: i32) -> Vec<Vec<i32>> {
        if r as usize * c as usize != mat.len() * mat[0].len() {
            return mat
        }
        let flat = mat.into_iter()
            .flatten()
            .collect::<Vec<_>>();
      
        flat.chunks(c as usize)
            .map(|s| s.to_vec())
            .collect()
    }
}

// Runtime: 0 ms, Beats 100.00%
// Memory: 2.42 MB, Beats 5.88%
