impl Solution {
    pub fn is_monotonic(nums: Vec<i32>) -> bool {
        //using simple one pass.
        let mut is_increasing = true;
        let mut is_decreasing = true;
        for (current_index, current_num) in nums.iter().enumerate() {
            if let Some(next_num) = nums.get(current_index + 1) {
                if *next_num > *current_num {
                    is_decreasing = false;
                    continue;
                }
                if *next_num < *current_num {
                    is_increasing = false;
                }
            }
        }
        is_decreasing | is_increasing
    }
}
