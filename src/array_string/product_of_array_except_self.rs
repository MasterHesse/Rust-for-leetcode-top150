pub struct Solution;

impl Solution {
    pub fn product_except_self(nums: Vec<i32>) -> Vec<i32> {
        let mut answer: Vec<i32> = vec![];
        let mut prefix: Vec<i32> = vec![];
        let mut suffix: Vec<i32> = vec![];
        prefix.push(1);
        suffix.push(1);

        for element in &nums[0..nums.len() - 1] {
            let temp = prefix.last();
            prefix.push(*element * *temp.unwrap());
        }

        for element in nums[1..].iter().rev() {
            let temp = suffix.last();
            suffix.push(*element * *temp.unwrap());
        }
        suffix.reverse();

        for i in 0..nums.len() {
            answer.push(prefix[i] * suffix[i]);
        }

        answer
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test1() {
        let nums = vec![1, 2, 3, 4];
        let answer = vec![24, 12, 8, 6];
        let expect = Solution::product_except_self(nums);
        assert_eq!(expect, answer);
    }

    #[test]
    fn test2() {
        let nums = vec![-1, 1, 0, -3, 3];
        let answer = vec![0, 0, 9, 0, 0];
        let expect = Solution::product_except_self(nums);
        assert_eq!(expect, answer);
    }
}
