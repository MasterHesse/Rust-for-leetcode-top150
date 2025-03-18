use std::collections::HashMap;

pub struct Solution;

impl Solution {
    pub fn word_pattern(pattern: String, s: String) -> bool {
        let words: Vec<&str> = s.split_whitespace().collect();
        if pattern.len() != words.len() {
            return false; // 长度不匹配，直接返回 false
        }

        let mut p_to_w: HashMap<char, &str> = HashMap::new();
        let mut w_to_p: HashMap<&str, char> = HashMap::new();

        for (ch, word) in pattern.chars().zip(words.iter()) {
            // 检查 pattern -> word 映射
            if let Some(&mapped_word) = p_to_w.get(&ch) {
                if mapped_word != *word {
                    return false;
                }
            } else {
                p_to_w.insert(ch, word);
            }

            // 检查 word -> pattern 反向映射
            if let Some(&mapped_char) = w_to_p.get(word) {
                if mapped_char != ch {
                    return false;
                }
            } else {
                w_to_p.insert(word, ch);
            }
        }

        true
    }
}
