impl Solution {
    pub fn find_restaurant(list1: Vec<String>, list2: Vec<String>) -> Vec<String> {
        let (shortest, longest) = if list1.len() <= list2.len() {
    (&list1[..], &list2[..])
} else {
    (&list2[..], &list1[..])
};

        let mut common_strings = vec![];
        for sum in 0..=longest.len() - 1 + shortest.len() - 1 {
            for i in 0..longest.len() {
                let j = sum - i;
                if j < 0 {break}
                if j >= shortest.len() {continue}
                
                if longest[i] == shortest[j] {common_strings.push( longest[i].clone() );}
            }
            if common_strings.len() > 0 {break}
        }
    common_strings
    }
}

// Runtime: 13 ms, Beats 8.70%
// Memory: 2.34 MB, Beats 100.00%
