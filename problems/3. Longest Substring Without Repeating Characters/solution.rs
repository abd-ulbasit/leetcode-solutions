impl Solution {
    pub fn length_of_longest_substring(s: String) -> i32 {
        let mut max_lenght = 0;
        let mut lenght = 0;
        let mut start_index: u32 = 0;
        let mut indecies: [i32; 256] = [-1; 256];
        let s = s.clone().into_bytes();
        for (index, character) in s.iter().enumerate() {
            if indecies[*character as usize] == -1 {
                indecies[*character as usize] = index as i32;
                lenght += 1;
                if lenght > max_lenght {
                    max_lenght = lenght;
                }
            } else {
                indecies[*character as usize] = index as i32;
                lenght += 1;
                //reset all as -1 till new char;
                // for i in start_index..
                while s[start_index as usize] != *character {
                    indecies[s[start_index as usize] as usize] = -1;
                    start_index += 1;
                    lenght -= 1;
                }
                start_index += 1;
                lenght -= 1;
            }
        }
        max_lenght
    }
}
