pub struct Solution;

impl Solution {
    pub fn remove_element(nums: &mut Vec<i32>, val: i32) -> i32 {
        nums.retain(|x| *x != val);

        let k: i32 = nums.len().try_into().unwrap();

        k
    }
}

#[cfg(test)]
mod test {
    use std::vec;

    use super::Solution;

    #[test]
    fn test() {
        std::env::set_var("RUST_BACKTRACE", "full");

        {
            let mut nums = vec![3, 2, 2, 3];
            let val = 3;
            let expected_nums = vec![2, 2];
            let k = Solution::remove_element(&mut nums, val);
            assert_eq!(k, expected_nums.len() as i32);

            let result: Vec<i32> = nums[..k as usize].to_vec();
            assert!(expected_nums.iter().all(|x| result.contains(x)));
        }

        {
            let mut nums = vec![0, 1, 2, 2, 3, 0, 4, 2];
            let val = 2;
            let expected_nums = vec![0, 1, 4, 0, 3];
            let k = Solution::remove_element(&mut nums, val);
            assert_eq!(k, expected_nums.len() as i32);

            let result: Vec<i32> = nums[..k as usize].to_vec();
            assert!(expected_nums.iter().all(|x| result.contains(x)));
        }
    }
}
