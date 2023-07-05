impl Solution {
    pub fn hammingWeight(n: u32) -> i32 {
        let (mut n, mut count) = (n, 0);
        while n > 0 {
            if (n & 1) == 1 {
                count += 1;
            }
            n >>= 1;
        }
        count
    }
}
