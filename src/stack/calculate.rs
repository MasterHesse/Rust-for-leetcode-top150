impl Solution {
    pub fn calculate(s: String) -> i32 {
        let mut stack: Vec<i32> = Vec::new();
        let mut res = 0;
        let mut num = 0;
        let mut sign = 1;

        let chars: Vec<char> = s.chars().collect();
        let n = chars.len();

        for i in 0..n {
            let c = chars[i];

            if c.is_digit(10) {
                num = num * 10 + (c as i32 - '0' as i32);
            } else if c == '+' || c == '-' {
                res += sign * num;
                num = 0;
                sign = if c == '+' { 1 } else { -1 };
            } else if c == '(' {
                stack.push(res);
                stack.push(sign);
                res = 0;
                sign = 1;
            } else if c == ')' {
                res += sign * num;
                num = 0;
                res *= stack.pop().unwrap();
                res += stack.pop().unwrap();
            }
        }

        res += sign * num;

        res
    }
}

pub struct Solution;
