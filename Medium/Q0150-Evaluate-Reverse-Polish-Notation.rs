impl Solution {
    pub fn eval_rpn(tokens: Vec<String>) -> i32 {
        let mut stack = Vec::with_capacity(200);
        for token in tokens {
            let first_char = token.as_bytes()[0];
            if token.len() > 1 || first_char > 47 {
                stack.push(token.parse::<i32>().unwrap());
                continue;
            }
            let rhs = stack.pop().unwrap();
            let lhs = stack.pop().unwrap();
            if token == "+" {
                stack.push(lhs + rhs);
            } else if token == "-" {
                stack.push(lhs - rhs);
            } else if token == "*" {
                stack.push(lhs * rhs);
            } else if token == "/" {
                stack.push(lhs / rhs);
            }
        }
        stack.pop().unwrap()
    }
}

// Runtime: 0 ms, Beats 100.00%
// Memory: 2.84 MB, Beats 23.62%
