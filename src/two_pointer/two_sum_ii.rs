pub struct Solution;

impl Solution {
    pub fn two_sum(numbers: Vec<i32>, target: i32) -> Vec<i32> {
        let mut left = 0;
        let mut right = numbers.len() - 1;

        while left < right {
            let sum = numbers[left] + numbers[right];

            if sum == target {
                return vec![(left + 1) as i32, (right + 1) as i32];
            } else if sum < target {
                left += 1;
            } else {
                right -= 1;
            }
        }
        vec![]
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test1() {
        let numbers = vec![2, 7, 11, 15];
        let target = 9;
        let answer = vec![1, 2];
        let expect = Solution::two_sum(numbers, target);
        assert_eq!(answer, expect);
    }

    #[test]
    fn test2() {
        let numbers = vec![2, 3, 4];
        let target = 6;
        let answer = vec![1, 3];
        let expect = Solution::two_sum(numbers, target);
        assert_eq!(answer, expect);
    }

    #[test]
    fn test3() {
        let numbers = vec![-1, 0];
        let target = -1;
        let answer = vec![1, 2];
        let expect = Solution::two_sum(numbers, target);
        assert_eq!(answer, expect);
    }
}
