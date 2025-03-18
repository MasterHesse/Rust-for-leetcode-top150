use std::collections::HashMap;

pub struct Solution;

impl Solution {
    pub fn is_isomorphic(s: String, t: String) -> bool {
        let mut s_to_t: HashMap<char, char> = HashMap::new();
        let mut t_to_s: HashMap<char, char> = HashMap::new();

        for (ch_s, ch_t) in s.chars().zip(t.chars()) {
            if let Some(&mapped) = s_to_t.get(&ch_s) {
                if mapped != ch_t {
                    return false;
                }
            } else {
                s_to_t.insert(ch_s, ch_t);
            }

            if let Some(&mapped) = t_to_s.get(&ch_t) {
                if mapped != ch_s {
                    return false;
                }
            } else {
                t_to_s.insert(ch_t, ch_s);
            }
        }

        true
    }
}
