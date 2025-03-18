use std::collections::HashMap;

pub struct Solution;

impl Solution {
    pub fn remove_duplicates(nums: &mut Vec<i32>) -> i32 {
        let mut map: HashMap<i32, bool> = HashMap::new();
        let mut k = 0;
        let mut expect = vec![];

        for element in nums.iter() {
            if map.get(element) == None {
                map.insert(*element, true);
                k += 1;

                expect.push(*element);
            }
        }

        *nums = expect;
        k
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test() {
        std::env::set_var("RUST_BACKTRACE", "full");

        {
            let mut nums = vec![1, 1, 2];
            let expected_nums = [1, 2];
            let k = Solution::remove_duplicates(&mut nums);
            assert_eq!(k, expected_nums.len().try_into().unwrap());
        }

        {
            let mut nums = vec![0, 0, 1, 1, 1, 2, 2, 3, 3, 4];
            let expected_nums = [0, 1, 2, 3, 4];
            let k = Solution::remove_duplicates(&mut nums);
            assert_eq!(k, expected_nums.len().try_into().unwrap());
        }
    }
}
