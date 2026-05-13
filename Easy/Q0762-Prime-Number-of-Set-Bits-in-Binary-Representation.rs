impl Solution {
    pub fn count_prime_set_bits(left: i32, right: i32) -> i32 {
        // max value = 10^6, representable with 20 bits
        // primes from 1-20 = 2, 3, 5, 7, 11, 13, 17, 19
        const IS_PRIME: [bool; 21] = [
            false, false, true,  true,  false, true,  false, // 0 - 6
            true,  false, false, false, true,  false, true,  // 7 - 13
            false, false, false, true,  false, true,  false, // 14 - 20
        ];
        (left..=right)
            .filter(|num| IS_PRIME[num.count_ones() as usize])
            .count() as i32
    }
}

// Runtime: 1 ms, Beats 57.58%
// Memory: 2.04 MB, Beats 78.79%
