pub struct Solution;

// Solution 1: VecDeque
/*
use std::collections::VecDeque;
impl Solution {
    pub fn rotate(nums: &mut Vec<i32>, k: i32) {
        let mut queue: VecDeque<i32> = VecDeque::from(nums.clone());
        let mut count = k;

        while count > 0 {
            let temp = queue.pop_back();
            queue.push_front(temp.unwrap());

            count -= 1;
        }

        *nums = Vec::from(queue);
    }
}
     */

// Solution 2: Reverse
impl Solution {
    pub fn rotate(nums: &mut Vec<i32>, k: i32) {
        if k == 0 {
            return;
        }
        let n = nums.len();
        let k = k as usize % n;
        if k == 0 {
            return;
        }
        nums.reverse();
        nums[..k].reverse();
        nums[k..].reverse();
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test() {
        {
            let mut nums = vec![1, 2, 3, 4, 5, 6, 7];
            let k = 3;
            let expect = vec![5, 6, 7, 1, 2, 3, 4];
            Solution::rotate(&mut nums, k);
            assert_eq!(nums, expect);
        }

        {
            let mut nums = vec![-1, -100, 3, 99];
            let k = 2;
            let expect = vec![3, 99, -1, -100];
            Solution::rotate(&mut nums, k);
            assert_eq!(nums, expect);
        }
    }
}
