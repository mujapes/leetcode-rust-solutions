impl Solution {
    pub fn unique_paths_with_obstacles(obstacle_grid: Vec<Vec<i32>>) -> i32 {
        let mut n = vec![1; obstacle_grid[0].len()];
        for i in 0..obstacle_grid.len() {
            for j in 0..obstacle_grid[0].len() {
                if obstacle_grid[i][j] == 1 {
                    if i == 0 {
                        n[j..].fill(0);
                        break
                    }
                    n[j] = 0;
                } else if i > 0 && j > 0 {
                    n[j] += n[j-1];
                }
            }    
        }
        *n.last().unwrap_or(&0)
    }
}

// Runtime: 0 ms, Beats 100.00%
// Memory: 2.21 MB, Beats 37.97%
