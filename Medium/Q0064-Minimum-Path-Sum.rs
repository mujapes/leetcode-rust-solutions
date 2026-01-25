impl Solution {
    pub fn min_path_sum(grid: Vec<Vec<i32>>) -> i32 {
        let mut row = vec![i32::MAX; grid[0].len()];
        row[0] = 0;
        for m in 0..grid.len() {
            for n in 0..grid[0].len() {
                if n == 0 {row[n] += grid[m][n];}
                else      {row[n] = grid[m][n] + row[n].min(row[n-1]);}
            }    
        }
        *row.last().unwrap_or(&-1)
    }
}

// Runtime: 0 ms, Beats 100.00%
// Memory: 2.58 MB, Beats 67.07%
