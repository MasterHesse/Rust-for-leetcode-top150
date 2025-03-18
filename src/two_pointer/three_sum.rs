pub struct Solution;

impl Solution {
    pub fn three_sum(mut nums: Vec<i32>) -> Vec<Vec<i32>> {
        let mut res: Vec<Vec<i32>> = Vec::new();
        nums.sort();

        for i in 0..nums.len() {
            if i > 0 && nums[i] == nums[i - 1] {
                continue;
            }

            let target = -nums[i];
            let mut left = i + 1;
            let mut right = nums.len() - 1;
            while left < right {
                let sum = nums[left] + nums[right];
                if sum == target {
                    res.push(vec![nums[i], nums[left], nums[right]]);
                    left += 1;
                    right -= 1;

                    while left < right && nums[left] == nums[left - 1] {
                        left += 1;
                    }
                    if right != nums.len() - 1 {
                        while left < right && nums[right] == nums[right + 1] {
                            right -= 1;
                        }
                    }
                } else if sum < target {
                    left += 1;
                } else {
                    right -= 1;
                }
            }
        }

        res
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test1() {
        let nums = vec![-1, 0, 1, 2, -1, -4];
        let answer: Vec<Vec<i32>> = vec![vec![-1, -1, 2], vec![-1, 0, 1]];
        let expect: Vec<Vec<i32>> = Solution::three_sum(nums);
        assert_eq!(answer, expect);
    }

    #[test]
    fn test2() {
        let nums = vec![0, 1, 1];
        let answer: Vec<Vec<i32>> = vec![];
        let expect: Vec<Vec<i32>> = Solution::three_sum(nums);
        assert_eq!(answer, expect);
    }

    #[test]
    fn test3() {
        let nums = vec![0, 0, 0];
        let answer: Vec<Vec<i32>> = vec![vec![0, 0, 0]];
        let expect: Vec<Vec<i32>> = Solution::three_sum(nums);
        assert_eq!(answer, expect);
    }
}
