pub struct Solution;

impl Solution {
    pub fn reverse_words(s: String) -> String {
        // 步骤 1: 去掉头尾空格，并用 split_whitespace 分割单词
        let words: Vec<&str> = s.split_whitespace().collect();

        // 步骤 2: 反转单词的顺序
        let reversed: Vec<&str> = words.into_iter().rev().collect();

        // 步骤 3: 用单个空格拼接
        reversed.join(" ")
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test1() {
        let s = String::from("the sky is blue");
        let expect = String::from("blue is sky the");
        let result = Solution::reverse_words(s);
        assert_eq!(expect, result);
    }

    #[test]
    fn test2() {
        let s = String::from("  hello world  ");
        let expect = String::from("world hello");
        let result = Solution::reverse_words(s);
        assert_eq!(expect, result);
    }

    #[test]
    fn test3() {
        let s = String::from("a good   example");
        let expect = String::from("example good a");
        let result = Solution::reverse_words(s);
        assert_eq!(expect, result);
    }
}
