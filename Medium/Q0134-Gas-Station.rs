impl Solution {
    pub fn can_complete_circuit(gas: Vec<i32>, cost: Vec<i32>) -> i32 {
        // total cost must be less than total gas available
        // starting station must have gas >= cost
        // keep track of total gas-cost and the minimum such value
        // if by the end the total is >= 0 there exists a solution?
        // prefix sum arrays?
        // solution is unique
        let mut remaining_gas = 0;
        let mut min_remaining = 10001;
        let mut min_idx = 0;
        for i in 0..gas.len() {
            remaining_gas += gas[i] - cost[i];
            if remaining_gas < min_remaining {
                min_remaining = remaining_gas;
                min_idx = i;
            }
        }
        if remaining_gas < 0 {return -1;}
        if min_idx == gas.len()-1 {return 0;}
        return min_idx as i32 + 1;
    }
}

// Runtime: 0 ms, Beats 100.00%
// Memory: 3.42 MB, Beats 7.49%
