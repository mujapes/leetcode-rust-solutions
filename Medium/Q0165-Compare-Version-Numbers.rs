impl Solution {
    pub fn compare_version(version1: String, version2: String) -> i32 {
        let v1_revisions: Vec<_> = version1.split('.').collect();
        let v2_revisions: Vec<_> = version2.split('.').collect();
        for i in 0..v1_revisions.len() {
            let v1_revision = v1_revisions[i].parse::<i32>().unwrap();
            let mut v2_revision = 0;
            if i < v2_revisions.len() {
                v2_revision = v2_revisions[i].parse::<i32>().unwrap();
            }
            if v1_revision > v2_revision { return 1; }
            if v2_revision > v1_revision { return -1; }
        }
        if v2_revisions.len() > v1_revisions.len() {
            for j in v1_revisions.len()..v2_revisions.len() {
                if v2_revisions[j].parse::<i32>().unwrap() > 0 {
                    return -1;
                }
            }
        }
        0
    }
}

// Runtime: 0 ms, Beats 100.00%
// Memory: 2.08 MB, Beats 90.63%
