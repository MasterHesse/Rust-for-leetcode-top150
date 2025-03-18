impl Solution {
    pub fn is_valid(s: String) -> bool {
        if s.len() == 0 {
            return false;
        }

        let mut stack: Vec<char> = Vec::new();
        let open: Vec<char> = vec!['(', '[', '{'];
        let close: Vec<char> = vec![')', ']', '}'];

        for ch in s.chars() {
            if open.contains(&ch) {
                stack.push(ch.clone());
            }
            if close.contains(&ch) {
                if stack.is_empty() {
                    return false;
                }
                if ch == ')' && *stack.last().unwrap() != '(' {
                    return false;
                }
                if ch == ']' && *stack.last().unwrap() != '[' {
                    return false;
                }
                if ch == '}' && *stack.last().unwrap() != '{' {
                    return false;
                }
                stack.pop();
            }
        }

        if stack.is_empty() {
            return true;
        } else {
            return false;
        }
    }
}

pub struct Solution;
