use std::collections::{HashSet, VecDeque};

impl Solution {
    pub fn ladder_length(begin_word: String, end_word: String, word_list: Vec<String>) -> i32 {
        let word_set: HashSet<String> = word_list.clone().into_iter().collect();
        if !word_set.contains(&end_word) {
            return 0;
        }

        let mut queue = VecDeque::new();
        queue.push_back((begin_word.clone(), 1));
        let mut visited = HashSet::new();
        visited.insert(begin_word);

        let mut chars_set = HashSet::new();
        for word in word_list {
            for ch in word.chars().into_iter() {
                chars_set.insert(ch);
            }
        }
        let chars: Vec<char> = chars_set.iter().cloned().collect();

        while let Some((curr, steps)) = queue.pop_front() {
            if curr == end_word {
                return steps;
            }

            for i in 0..curr.len() {
                for &c in &chars {
                    if c != curr.chars().nth(i).unwrap() {
                        let mut next = curr.chars().collect::<Vec<char>>();
                        next[i] = c;
                        let next_str: String = next.into_iter().collect();

                        if word_set.contains(&next_str) && !visited.contains(&next_str) {
                            visited.insert(next_str.clone());
                            queue.push_back((next_str, steps + 1));
                        }
                    }
                }
            }
        }

        0
    }
}

pub struct Solution;
