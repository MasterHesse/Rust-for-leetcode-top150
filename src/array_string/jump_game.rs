pub struct Solution;

impl Solution {
    pub fn can_jump(nums: Vec<i32>) -> bool {
        let len = nums.len();
        let mut max_position = 0;

        if len as i32 == 1 {
            return true;
        } else if nums[0] == 0 {
            return false;
        } else {
            for i in 0..len {
                if i > max_position {
                    return false;
                }

                if i + nums[i] as usize > max_position {
                    max_position = i + nums[i] as usize;
                }

                if max_position >= len - 1 {
                    return true;
                }
            }
        }
        false
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;
    #[test]
    fn test() {
        {
            let nums = vec![2, 3, 1, 1, 4];
            let result = Solution::can_jump(nums);
            let expect = true;
            assert_eq!(result, expect);
        }

        {
            let nums = vec![3, 2, 1, 0, 4];
            let result = Solution::can_jump(nums);
            let expect = false;
            assert_eq!(result, expect);
        }
    }
}
