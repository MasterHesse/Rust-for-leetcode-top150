use std::collections::HashMap;

impl Solution {
    pub fn group_anagrams(strs: Vec<String>) -> Vec<Vec<String>> {
        let mut map: HashMap<Vec<i32>, Vec<String>> = HashMap::new();

        for s in strs {
            let mut freq = [0; 26];

            for c in s.chars() {
                freq[(c as u8 - b'a') as usize] += 1
            }

            map.entry(freq.to_vec()).or_insert(Vec::new()).push(s);
        }

        map.into_values().collect()
    }
}

pub struct Solution;
