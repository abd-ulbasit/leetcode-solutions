impl Solution {
    pub fn search_insert(nums: Vec<i32>, target: i32) -> i32 {
        Solution::search(0, nums.len() - 1, target, nums)
    }
    pub fn search(start: usize, end: usize, target: i32, nums: Vec<i32>) -> i32 {
        if start == end {
            if nums[start] >= target {
                return start as i32;
            } else {
                return start as i32 + 1;
            }
        }
        let mid = (start + end) / 2;
        if nums[mid] == target {
            return mid as i32;
        } else if nums[mid] < target {
            Solution::search(if mid == end { mid } else { mid + 1 }, end, target, nums)
        } else {
            Solution::search(
                start,
                if mid == start { mid } else { mid - 1 },
                target,
                nums,
            )
        }
    }
}
