impl Solution {
    pub fn maximum_product(nums: Vec<i32>) -> i32 { 
        /*
        for all positives or negatives max product = product of 3 max nums
        if all negatives result will be negative so looking for the smallest absolute values (max nominal) to multiply
        for a mix = one number will be positive and other two both min negatives or max positives
        so need to track 3 largest and 2 smallest negative ints
        */
        let mut largest = vec![-1001; 3];
        let mut smallest_negative = vec![0; 2];
        for num in nums {
            if num > largest[0] {
                largest.insert(0, num);
            } else if num > largest[1] {
                largest.insert(1, num);
            } else if num  > largest[2] {
                largest.insert(2, num);
            }
            if num < smallest_negative[0] {
                smallest_negative.insert(0, num);
            } else if num < smallest_negative[1] {
                smallest_negative.insert(1, num);
            }
            largest.truncate(3);
            smallest_negative.truncate(2);
        }
        largest.iter()
            .product::<i32>()
            .max(
                smallest_negative.iter()
                    .product::<i32>() * largest[0]
            )
    }
}

// Runtime: 0 ms, Beats 100.00%
// Memory: 2.20 MB, Beats 100.00%
