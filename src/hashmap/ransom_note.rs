use std::collections::HashMap;

pub struct Solution;

impl Solution {
    pub fn can_construct(ransom_note: String, magazine: String) -> bool {
        let mut char_count: HashMap<char, i32> = HashMap::new();

        for ch in magazine.chars() {
            *char_count.entry(ch).or_insert(0) += 1;
        }

        for ch in ransom_note.chars() {
            if let Some(count) = char_count.get_mut(&ch) {
                if *count > 0 {
                    *count -= 1;
                } else {
                    return false;
                }
            } else {
                return false;
            }
        }
        true
    }
}
