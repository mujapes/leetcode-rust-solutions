impl Solution {
    pub fn set_zeroes(matrix: &mut Vec<Vec<i32>>) {
        let (mut row1, mut col1) = (false, false);
        for m in 0..matrix.len() {
            for n in 0..matrix[0].len() {
                if matrix[m][n] == 0 {
                    if m > 0 && n > 0 {
                        for prev in 0..m { matrix[prev][n] = 0; }
                        for prev in 0..n { matrix[m][prev] = 0; }
                    } else {
                        if m == 0 { row1 = true; }
                        if n == 0 { col1 = true; }
                    }
                } else if m > 0 && n > 0 && (matrix[m][0] == 0 || matrix[0][n] == 0) {
                    matrix[m][n] = 0;
                }
            }
        }
        if row1 { for n in 0..matrix[0].len() {matrix[0][n] = 0;} }
        if col1 { for m in 0..matrix.len() {matrix[m][0] = 0;} }
    }
}

// Runtime: 0 ms, Beats 100.00%
// Memory: 2.59 MB, Beats 21.00%
