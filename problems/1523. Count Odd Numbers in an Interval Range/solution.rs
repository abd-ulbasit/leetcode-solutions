impl Solution {
    pub fn count_odds(low: i32, high: i32) -> i32 {
        if high % 2 == 0 && low % 2 == 0 {
            (high - low) / 2
        } else {
            (high - low) / 2 + 1
        }
    }
}
