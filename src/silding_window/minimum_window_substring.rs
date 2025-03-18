pub struct Solution;

use std::collections::HashMap;

impl Solution {
    pub fn min_window(s: String, t: String) -> String {
        if s.is_empty() || t.is_empty() {
            return "".to_string();
        }

        // Step 1: 统计 t 中每个字符的频率
        let mut t_count = HashMap::new();
        for c in t.chars() {
            *t_count.entry(c).or_insert(0) += 1;
        }

        let mut windows_count = HashMap::new();
        let required = t_count.len(); // 不同字符的种类数
        let mut left = 0;
        let mut right = 0;
        let mut formed = 0; // 当前满足条件的字符种类数
        let mut min_len = usize::MAX;
        let mut min_window = (0, 0);

        let s_chars: Vec<char> = s.chars().collect();

        while right < s_chars.len() {
            // 扩展窗口
            let c = s_chars[right];
            *windows_count.entry(c).or_insert(0) += 1;

            // 如果当前字符的频率满足 t 中的要求
            if let Some(&count) = t_count.get(&c) {
                if windows_count[&c] == count {
                    formed += 1;
                }
            }

            // 收缩窗口
            while left <= right && formed == required {
                // 更新最小窗口
                if right - left + 1 < min_len {
                    min_len = right - left + 1;
                    min_window = (left, right);
                }

                // 左指针移除字符
                let c = s_chars[left];
                *windows_count.get_mut(&c).unwrap() -= 1;

                if let Some(&count) = t_count.get(&c) {
                    if windows_count[&c] < count {
                        formed -= 1;
                    }
                }
                left += 1;
            }

            right += 1;
        }

        // 如果没找到合法窗口，返回空字符串
        if min_len == usize::MAX {
            return "".to_string();
        }
        s_chars[min_window.0..=min_window.1].iter().collect()
    }
}
