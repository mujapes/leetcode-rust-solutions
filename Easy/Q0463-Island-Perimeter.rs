impl Solution {
    pub fn island_perimeter(grid: Vec<Vec<i32>>) -> i32 {
        let mut res = 0;
        for row in 0..grid.len() {
            for col in 0..grid[0].len() {
                if grid[row][col] == 1 {
                    res += 4;
                    if row > 0 && grid[row-1][col] == 1 { res -= 2; }
                    if col > 0 && grid[row][col-1] == 1 { res -= 2; }
                }
            }
        }
        res
    }
}

// Runtime: 1 ms, Beats 64.71%
// Memory: 2.22 MB, Beats 72.55%
