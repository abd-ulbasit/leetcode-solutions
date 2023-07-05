use std::cmp::Ordering;
impl Solution {
    pub fn is_monotonic(nums: Vec<i32>) -> bool {
        match nums.last().unwrap().cmp(nums.first().unwrap()) {
            Ordering::Greater => {
                //inferring that its monotonic increasing & then looping over all its elements to check
                for (current_index, current_num) in nums.iter().enumerate() {
                    if let Some(next_num) = nums.get(current_index + 1) {
                        if *next_num < *current_num {
                            return false;
                        }
                    }
                }
                return true;
            }
            Ordering::Less => {
                //inferring that its monotonic decreasing & then looping over all its elements to check
                for (current_index, current_num) in nums.iter().enumerate() {
                    if let Some(next_num) = nums.get(current_index + 1) {
                        if *next_num > *current_num {
                            return false;
                        }
                    }
                }
                return true;
            }
            Ordering::Equal => {
                //inferring that its all elements are same & then looping over all its elements to check
                for (current_index, current_num) in nums.iter().enumerate() {
                    if let Some(next_num) = nums.get(current_index + 1) {
                        if *next_num != *current_num {
                            return false;
                        }
                    }
                }
                return true;
            }
        }
    }
}
