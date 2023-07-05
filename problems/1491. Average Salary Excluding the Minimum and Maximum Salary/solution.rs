use std::i32::MAX;
impl Solution {
    pub fn average(salary: Vec<i32>) -> f64 {
        let mut min = MAX;
        let mut max = 0;
        let mut sum: u32 = 0;
        let mut len: u16 = 0;
        for (index, i_salary) in salary.iter().enumerate() {
            len += 1;
            sum = sum + (*i_salary as u32);
            if *i_salary < min {
                min = salary[index];
            }
            if *i_salary > max {
                max = salary[index];
            }
        }
        (sum - (max as u32 + min as u32)) as f64 / (len - 2) as f64
    }
}
