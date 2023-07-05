use std::cmp::Ordering;
impl Solution {
    pub fn three_sum(nums: Vec<i32>) -> Vec<Vec<i32>> {
        //using two pointers:
        let mut nums = nums;
        if nums.len() < 3 {
            return vec![];
        };
        let mut set = Vec::new();
        nums.sort_unstable();
        for i in 0..nums.len() - 2 {
            if nums[i] > 0 {
                break;
            }
            let mut low = i + 1;
            let mut high = nums.len() - 1;
            while high > low {
                match (nums[i] + nums[low] + nums[high]).cmp(&0) {
                    Ordering::Greater => {
                        high -= 1;
                        while let Some(h) = nums.get(high) {
                            if *h == nums[high + 1] {
                                high -= 1;
                            } else {
                                break;
                            }
                        }
                        // continue;
                    }
                    Ordering::Less => {
                        low += 1;
                        while let Some(l) = nums.get(low) {
                            if *l == nums[low - 1] {
                                low += 1;
                            } else {
                                break;
                            }
                        }
                        // continue;
                    }
                    Ordering::Equal => {
                        if !set.contains(&vec![nums[i], nums[low], nums[high]]) {
                            set.push(vec![nums[i], nums[low], nums[high]]);
                        }
                        low += 1
                        // break;
                    }
                }
            }
        }
        set
    }
}
