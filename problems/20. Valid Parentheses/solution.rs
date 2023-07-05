impl Solution {
    pub fn is_valid(s: String) -> bool {
        let mut stack = Vec::new();
        for x in s.chars() {
            match x {
                '(' => stack.push('('),
                '{' => stack.push('{'),
                '[' => stack.push('['),
                '}' => {
                    if stack.pop().unwrap_or_else(|| '?') != '{' {
                        return false;
                    }
                }
                ']' => {
                    if stack.pop().unwrap_or_else(|| '?') != '[' {
                        return false;
                    }
                }
                ')' => {
                    if stack.pop().unwrap_or_else(|| '?') != '(' {
                        return false;
                    }
                }
                _ => continue,
            }
        }
        if stack.pop().is_none() {
            return true;
        } else {
            return false;
        }
    }
}
