impl Solution {
    pub fn ways_to_reach_stair(k: i32) -> i32 {
        // special cases where 'dist' can be greater than 2^('next_pow'-1)
        if k == 0 {return 2;}
        if k == 1 {return 4;}
        if k < 5 {return 6-k;}
        // jump x times = add 2^x -1 = add u32::MAX >> 32-x 
        // so min(x) = ceil(log2(K))
        // for k = 2^a trivial x = q-1,
        // backtrack y times
        // if 2^ceil(log2(K)) - k > ceil(log2(K)) + 1, ways = 0
        // upper bound power of 2
        let mut next_pow = 0;
        while 1 << next_pow < k {next_pow += 1;}
        // distance from upper bound
        let dist = (1 << next_pow) - k;
        // *dist is the amount of backtraces we need = y
        if dist == 0 {return 1;}
        if dist > next_pow + 1 {return 0;}
        // for all combinations of y backtraces in x + y steps we have (x+y)!/y!(x+y-y)! = (x+y)!/y!x! = prod(x+1..=x+y)/y!
        // from these we only need combinations where there are no adjacent backsteps so we only have x+1 possible slots for backtraces (after a jump or at the start)
        // so (x+1)!/y!(x+1-y)! = prod(x+2-y..=x+1)/y!
        // factorials can easily lead to overflows so we should multiply iteratively where possible.
        let mut combinations = 1.0;
        let mut mult = next_pow+2-dist;
        // y! has y multiplications as does prod(x+2-y..=x+1) 
        for divisor in 1..=dist {
            combinations *= mult as f64;
            combinations /= divisor as f64;
            mult += 1;
        }
        combinations.round() as i32
    }
}

// Runtime: 0 ms, Beats 100.00%
// Memory: 2.08 MB, Beats 100.00%
