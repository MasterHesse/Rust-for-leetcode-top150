use std::char;
use std::collections::HashSet;

pub struct Solution;

impl Solution {
    pub fn length_of_longest_substring(s: String) -> i32 {
        let mut char_set =HashSet::new();
        let mut start = 0;
        let mut max_len = 0;
        let chars: Vec<char> = s.chars().collect();

        for end in 0..chars.len()  {
            while char_set.contains(&chars[end]) {
                if start < chars.len() {
                    char_set.remove(&chars[start]); // 从左侧移除字符
                    start += 1; // 缩小窗口
                }
            }

            char_set.insert(chars[end]);

            max_len = max_len.max(end - start + 1);
        }

        max_len as i32
    }
}