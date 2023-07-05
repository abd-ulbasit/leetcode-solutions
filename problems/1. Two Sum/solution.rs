use std::collections::HashMap;

impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut seen = HashMap::new();
        for (index, num) in nums.iter().enumerate() {
            if let Some(x) = seen.get(num) {
                return vec![index as i32, *x];
            } else {
                seen.insert(target - *num, index as i32);
            }
        }
        vec![]
    }
}
