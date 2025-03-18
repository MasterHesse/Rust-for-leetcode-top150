use std::collections::HashMap;
use std::result;

pub struct Solution;

impl Solution {
    pub fn find_substring(s: String, words: Vec<String>) -> Vec<i32> {
        let word_len = words[0].len(); // 每个单词的长度
        let word_count = words.len(); // 单词数量
        let substring_len = word_len * word_count; // 子串的总长度
        let mut result = vec![];

        if s.len() < substring_len {
            return result; // 如果 s 的长度小于所需的子串长度，直接返回空结果
        }

        // 1. 统计 words 中每个单词的频率
        let mut word_map = HashMap::new();
        for word in &words {
            *word_map.entry(word.clone()).or_insert(0) += 1;
        }

        // 2. 遍历每个可能的起始点（0 到 word_len - 1）
        for i in 0..word_len {
            let mut left = i; // 窗口的左边界
            let mut right = i; // 窗口的右边界
            let mut current_map = HashMap::new(); // 当前窗口内单词频率

            while right + word_len <= s.len() {
                // 3. 从右边界提取一个单词
                let word = &s[right..right + word_len];
                right += word_len;

                if word_map.contains_key(word) {
                    // 如果单词在 words 中，更新当前窗口的频率
                    *current_map.entry(word.to_string()).or_insert(0) += 1;

                    // 如果某个单词的频率超出限制，缩小窗口
                    while current_map[word] > *word_map.get(word).unwrap() {
                        let left_word = &s[left..left + word_len];
                        *current_map.get_mut(left_word).unwrap() -= 1;
                        if current_map[left_word] == 0 {
                            current_map.remove(left_word);
                        }
                        left += word_len;
                    }

                    // 如果窗口大小正好等于子串长度，记录起始索引
                    if right - left == substring_len {
                        result.push(left as i32);
                    }
                } else {
                    // 如果单词不在 words 中，清空当前窗口
                    current_map.clear();
                    left = right;
                }
            }
        }

        result
    }
}
