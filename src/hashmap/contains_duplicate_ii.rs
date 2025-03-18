use std::collections::HashMap;

impl Solution {
    pub fn contains_nearby_duplicate(nums: Vec<i32>, k: i32) -> bool {
        let mut map: HashMap<i32, usize> = HashMap::new();

        for i in 0..nums.len() {
            if !map.contains_key(&nums[i]) {
                map.insert(nums[i], i);
            } else {
                if i32::abs(*map.get(&nums[i]).unwrap() as i32 - i as i32) <= k {
                    return true;
                } else {
                    map.insert(nums[i], i);
                }
            }
        }

        false
    }
}

pub struct Solution;
