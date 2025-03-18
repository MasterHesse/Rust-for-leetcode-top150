pub struct Solution;

impl Solution {
    pub fn is_palindrome(s: String) -> bool {
        // 转换为小写，过滤非字母和数字的字符，收集为新字符串
        let filtered: String = s
            .chars()
            .filter(|c| c.is_ascii_alphanumeric()) // 只保留字母和数字
            .map(|c| c.to_ascii_lowercase()) // 转换为小写
            .collect(); // 收集为字符串

        // 获取反转后的字符串
        let rev: String = filtered.chars().rev().collect();

        // 比较原始字符串和反转字符串
        filtered == rev
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test1() {
        let s = String::from("A man, a plan, a canal: Panama");
        let answer = true;
        let expect = Solution::is_palindrome(s);
        assert_eq!(answer, expect);
    }

    #[test]
    fn test2() {
        let s = String::from("race a car");
        let answer = false;
        let expect = Solution::is_palindrome(s);
        assert_eq!(answer, expect);
    }

    #[test]
    fn test3() {
        let s = String::from(" ");
        let answer = true;
        let expect = Solution::is_palindrome(s);
        assert_eq!(answer, expect);
    }
}
