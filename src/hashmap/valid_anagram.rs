use std::collections::HashMap;

pub struct Solution;

impl Solution {
    pub fn is_anagram(s: String, t: String) -> bool {
        if s.len() != t.len() {
            return false;
        }

        let mut char_count: HashMap<char, i32> = HashMap::new();

        for ch in s.chars() {
            *char_count.entry(ch).or_insert(0) += 1;
        }

        for ch in t.chars() {
            let count = char_count.entry(ch).or_insert(0);
            *count -= 1;
            if *count < 0 {
                return false;
            }
        }

        true
    }
}
