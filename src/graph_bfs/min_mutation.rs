use std::collections::{HashSet, VecDeque};

impl Solution {
    pub fn min_mutation(start_gene: String, end_gene: String, bank: Vec<String>) -> i32 {
        let bank_set: HashSet<String> = bank.into_iter().collect();
        if !bank_set.contains(&end_gene) {
            return -1;
        }

        let mut queue = VecDeque::new();
        queue.push_back((start_gene.clone(), 0));
        let mut visited = HashSet::new();
        visited.insert(start_gene);

        let chars = ['A', 'C', 'G', 'T'];

        while let Some((curr, steps)) = queue.pop_front() {
            if curr == end_gene {
                return steps;
            }

            for i in 0..curr.len() {
                for &c in &chars {
                    if c != curr.chars().nth(i).unwrap() {
                        let mut next = curr.chars().collect::<Vec<char>>();
                        next[i] = c;
                        let next_str: String = next.into_iter().collect();

                        if bank_set.contains(&next_str) && !visited.contains(&next_str) {
                            visited.insert(next_str.clone());
                            queue.push_back((next_str, steps + 1));
                        }
                    }
                }
            }
        }
        -1
    }
}

pub struct Solution;
