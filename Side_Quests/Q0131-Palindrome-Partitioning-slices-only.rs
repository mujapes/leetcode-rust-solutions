pub fn partition(s: &str) -> Vec<Vec<&str>> {
    let mut parts: Vec<Vec<[u8; 2]>> = Vec::new();
    let mut trivial = Vec::with_capacity(s.len());

    for i in 0..s.len() {
        trivial.push([i as u8, 1]);
    }

    fn ptr_to_slice<'a>(ptr: &[u8; 2], s: &'a str) -> &'a str {
        &s[ptr[0] as usize..(ptr[0] + ptr[1]) as usize]
    }
    
    fn recurse(part: Vec<[u8; 2]>, parts: &mut Vec<Vec<[u8; 2]>>, s: &str, unpartitioned_start: usize) {
        parts.push(part.clone());
        for i in unpartitioned_start..part.len()-1 {
            // 2 element palindrom builder
            if part[i][1] == 1 
            && ptr_to_slice(&part[i+1], s) == ptr_to_slice(&part[i], s)
            {
                let mut new_part = Vec::with_capacity(part.len()-1);
                new_part.extend_from_slice(&part[0..i]);
                new_part.push([part[i][0], 2]);
                new_part.extend_from_slice(&part[i+2..part.len()]);
                recurse(new_part, parts, s, i);
            }
            // right hand side len will be 1 due to unpatitioned_start offsets
            if i > 0
            && ptr_to_slice(&part[i-1], s) == ptr_to_slice(&part[i+1], s)
            {
                // palindrome expansion
                let mut new_part = Vec::with_capacity(part.len()-2);
                new_part.extend_from_slice(&part[0..i-1]);
                new_part.push( [part[i-1][0], part[i][1] + 2] );
                new_part.extend_from_slice(&part[i+2..part.len()]);
                recurse(new_part, parts, s, i-1);
            }
        }
    }

    recurse(trivial, &mut parts, &s, 0);
    parts.into_iter()
        .map(|part| {
            part.into_iter()
                .map(|pal| ptr_to_slice(&pal, &s)) // 1. Modify the strings here
                .collect::<Vec<&str>>() // 2. CRITICAL: Pack them back into a row vector
        })
        .collect()

}

pub fn partition_old(s: String) -> Vec<Vec<String>> {
        // find non-trivial palindromes then recursively combine with all others
        // search for length 2 and length 3 palindromes as these will seed all larger onees
        // sort storage bounds by lower AND upper bounds?
        // data structure that quickly shows elements exluded by bounds
        // collapes bounds into one number
        
        // array of single chars then 
        let mut parts: Vec<Vec<String>> = Vec::new();
        let mut trivial = Vec::with_capacity(s.len());

        for c in s.chars() {
            trivial.push(c.to_string());
        }
        
        fn recurse(part: Vec<String>, parts: &mut Vec<Vec<String>>, unpartitioned_start: usize) {
            parts.push(part.clone());
            for i in unpartitioned_start..part.len()-1 {
                // 2 element palindrom builder
                if part[i].len() == 1 && part[i+1] == part[i] {
                    let mut new_part = Vec::with_capacity(part.len()-1);
                    new_part.extend_from_slice(&part[0..i]);
                    new_part.push( format!("{}{}", part[i], part[i]) );
                    new_part.extend_from_slice(&part[i+2..part.len()]);
                    recurse(new_part, parts, i);
                }
                if i > 0 {
                    // palindrome expansion
                    if part[i-1] == part[i+1].chars().rev().collect::<String>() {
                        let mut new_part = Vec::with_capacity(part.len()-2);
                        new_part.extend_from_slice(&part[0..i-1]);
                        new_part.push( format!("{}{}{}", part[i-1], part[i], part[i+1]) );
                        new_part.extend_from_slice(&part[i+2..part.len()]);
                        recurse(new_part, parts, i-1);
                    }
                }
            }
        }

        recurse(trivial, &mut parts, 0);
        parts.into_iter().collect()
    }

/*
input="aaaaaaaaaaaaaaaa" len=16 runs=20
  slices: parts=32768 avg_us=48617.80 min_us=47148 max_us=58970
          avg_peak_kb=5120.03 min_peak_kb=5120.03 max_peak_kb=5120.03
  old:    parts=32768 avg_us=100211.45 min_us=97001 max_us=127136
          avg_peak_kb=7808.75 min_peak_kb=7808.75 max_peak_kb=7808.75
  speedup old/slices = 2.06x
  memory old/slices = 1.53x

input="aaaaaaaaaaaaaaaaaa" len=18 runs=20
  slices: parts=131072 avg_us=201917.55 min_us=197140 max_us=244553
          avg_peak_kb=22528.03 min_peak_kb=22528.03 max_peak_kb=22528.03
  old:    parts=131072 avg_us=424312.75 min_us=411913 max_us=526126
          avg_peak_kb=34560.85 min_peak_kb=34560.85 max_peak_kb=34560.85
  speedup old/slices = 2.10x
  memory old/slices = 1.53x

input="aaaabbaaaaaabbaa" len=16 runs=20
  slices: parts=2879 avg_us=4915.15 min_us=4645 max_us=5820
          avg_peak_kb=539.50 min_peak_kb=539.50 max_peak_kb=539.50
  old:    parts=2879 avg_us=10234.25 min_us=9771 max_us=12289
          avg_peak_kb=806.97 min_peak_kb=806.97 max_peak_kb=806.97
  speedup old/slices = 2.08x
  memory old/slices = 1.50x

input="aalaaaajaataasaaga" len=18 runs=20
  slices: parts=780 avg_us=1579.70 min_us=1461 max_us=1966
          avg_peak_kb=171.53 min_peak_kb=171.53 max_peak_kb=171.53
  old:    parts=780 avg_us=3429.90 min_us=3173 max_us=4420
          avg_peak_kb=259.82 min_peak_kb=259.82 max_peak_kb=259.82
  speedup old/slices = 2.17x
  memory old/slices = 1.51x
*/