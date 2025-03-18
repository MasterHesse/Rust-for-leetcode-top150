use std::collections::HashSet;

pub struct Solution;

impl Solution {
    pub fn longest_consecutive(nums: Vec<i32>) -> i32 {
        if nums.is_empty() {
            return 0;
        }

        let mut num_set: HashSet<i32> = nums.iter().cloned().collect();
        let mut max_length = 0;

        for &num in &nums {
            if num_set.contains(&num) {
                num_set.remove(&num); // 先移除，避免重复查询
                let mut current_length = 1;
                let mut left = num - 1;
                let mut right = num + 1;

                // 扩展左边界
                while num_set.remove(&left) {
                    left -= 1;
                    current_length += 1;
                }

                // 扩展右边界
                while num_set.remove(&right) {
                    right += 1;
                    current_length += 1;
                }

                max_length = max_length.max(current_length);
            }
        }

        max_length
    }
}
