pub struct Solution;

impl Solution {
    pub fn is_subsequence(s: String, t: String) -> bool {
        if s.is_empty() {
            return true;
        }

        let mut s_rev: Vec<char> = s.chars().rev().collect();
        let t_vec: Vec<char> = t.chars().collect();

        for c in t_vec {
            if let Some(&last) = s_rev.last() {
                // 检查是否有元素
                if c == last {
                    s_rev.pop(); // 匹配成功，弹出最后一个字符
                }
            } else {
                break; // 如果 s_rev 已经空了，提前退出
            }
        }

        s_rev.is_empty() // 判断是否所有字符都被匹配
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test1() {
        let s = String::from("abc");
        let t = String::from("ahbgdc");
        let answer = true;
        let expect = Solution::is_subsequence(s, t);
        assert_eq!(answer, expect);
    }

    #[test]
    fn test2() {
        let s = String::from("axc");
        let t = String::from("ahbgdc");
        let answer = false;
        let expect = Solution::is_subsequence(s, t);
        assert_eq!(answer, expect);
    }
}
