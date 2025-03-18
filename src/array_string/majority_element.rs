pub struct Solution;

impl Solution {
    pub fn majority_element(nums: Vec<i32>) -> i32 {
        let mut candidate = 0;
        let mut count = 0;

        // 第一步：找出候选者
        for num in nums.iter() {
            if count == 0 {
                candidate = *num;
                count = 1;
            } else if *num == candidate {
                count += 1;
            } else {
                count -= 1;
            }
        }

        // 第二步：返回候选者
        candidate
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test() {
        std::env::set_var("RUST_BACKTRACE", "full");

        {
            let nums = vec![3, 2, 3];
            let expect = 3;
            assert_eq!(Solution::majority_element(nums), expect);
        }

        {
            let nums = vec![2, 2, 1, 1, 1, 2, 2];
            let expect = 2;
            assert_eq!(Solution::majority_element(nums), expect);
        }
    }
}
