impl Solution {
    pub fn fizz_buzz(n: i32) -> Vec<String> {
        (1..n+1).map( |i| {
            let mut res = String::new();
            if i % 3 == 0 {res += "Fizz"}
            if i % 5 == 0 {res += "Buzz"}
            if res == String::new() {return format!("{}", i)}
            res
        } ).collect()
    }
}

// Runtime: 0 ms, Beats 100.00%
// Memory: 2.77 MB, Beats 45.33%
