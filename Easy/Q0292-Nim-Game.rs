/* true
true
true
false
true
true
true
false
true
true
true

*/
impl Solution {
    pub fn can_win_nim(n: i32) -> bool {
        n % 4 != 0
    }
}

// Runtime: 0 ms, Beats 100.00%
// Memory: 2.22 MB, Beats -%
