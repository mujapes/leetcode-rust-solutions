impl Solution {
    pub fn min_cost_climbing_stairs(cost: Vec<i32>) -> i32 {
        let mut min_cost = vec![0_i32; cost.len()];
        for i in 2..cost.len() {
            min_cost[i] = (min_cost[i-2] + cost[i-2]).min(
            min_cost[i-1] + cost[i-1]
        );
        }
        (min_cost[cost.len()-2] + cost[cost.len()-2]).min(
            min_cost[cost.len()-1] + cost[cost.len()-1]
        ) 
    }
}

// Runtime: 0 ms, Beats 100.00%
// Memory: 2.17 MB, Beats 67.50%
