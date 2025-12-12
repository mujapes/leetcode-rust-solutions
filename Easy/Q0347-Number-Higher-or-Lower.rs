/** 
 * Forward declaration of guess API.
 * @param  num   your guess
 * @return 	     -1 if num is higher than the picked number
 *			      1 if num is lower than the picked number
 *               otherwise return 0
 * unsafe fn guess(num: i32) -> i32 {}
 */
impl Solution {
    unsafe fn guessNumber(n: i32) -> i32 {
        let (mut left, mut right) = (1, n);
        loop {
            let mid = left + (right-left)/2;
            let guess = guess(mid);
            if guess == 1 {
                (left, right) = (mid + 1, right);
            } else if guess == -1 {
                (left, right) = (left, mid - 1);
            } else {break mid}
        }
    }
}

// Runtime: 0 ms, Beats 100.00%
// Memory: 2.12 MB, Beats 85.71%
