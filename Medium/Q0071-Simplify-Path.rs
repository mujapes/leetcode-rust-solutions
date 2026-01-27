impl Solution {
    pub fn simplify_path(path: String) -> String {
        let mut dest = Vec::with_capacity( path.len()/4 );
        let mut head = 0;
        for i in 0..=path.len() {
            if i == path.len() || &path[i..=i] == "/" {
                if head + 2 == i && &path[i-2..i] == ".." {
                    dest.pop();
                } else if head + 1 < i || (head + 1 == i && &path[i-1..i] != ".") {
                    dest.push(&path[head..i]);
                }
                head = i+1;
            }
        }
        String::from("/") + &dest.join("/")
    }
}

// Runtime: 0 ms, Beats 100.00%
// Memory: 2.35 MB, Beats 45.00%
