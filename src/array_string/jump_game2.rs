pub struct Solution;

use std::cmp;
impl Solution {
    pub fn jump(nums: Vec<i32>) -> i32 {
        let mut farthest: i32 = 0;
        let mut current_end: i32 = 0;
        let mut count: i32 = 0;

        for i in 0..nums.len() {
            if i == nums.len() - 1 {
                break;
            }

            farthest = cmp::max(farthest, nums[i] + i as i32);

            if i as i32 == current_end {
                count += 1;
                current_end = farthest;
            }
        }

        count
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test() {
        std::env::set_var("RUST_BACKTRACE", "full");

        {
            let nums = vec![2, 3, 1, 1, 4];
            let result = Solution::jump(nums);
            let expect = 2;
            assert_eq!(result, expect);
        }

        {
            let nums = vec![2, 3, 0, 1, 4];
            let result = Solution::jump(nums);
            let expect = 2;
            assert_eq!(result, expect);
        }
    }
}
