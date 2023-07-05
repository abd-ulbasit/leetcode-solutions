impl Solution {
    pub fn str_str(haystack: String, needle: String) -> i32 {
        'outer: for (index_i, i) in haystack.bytes().enumerate() {
            if i != needle.as_bytes()[0] {
                continue 'outer;
            } else {
                for (index_j, j) in needle.bytes().enumerate() {
                    //getting the Byte of haystack;
                    let k = haystack.as_bytes().get(index_i + index_j);
                    if let Some(k) = k {
                        if j != *k {
                            continue 'outer;
                        }
                    } else {
                        continue 'outer;
                    }
                }
                return index_i as i32;
            }
        }
        -1
        //O(n*2)
    }
}
