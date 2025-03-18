use core::num;
use std::usize;

pub struct Solution;

impl Solution {    
    pub fn min_sub_array_len(target: i32, nums: Vec<i32>) -> i32 {
        let mut start = 0;
        let mut sum = 0;
        let mut min_len = usize::MAX;

        for end in 0..nums.len() {
            sum += nums[end];
            
            while sum >= target {
                min_len = min_len.min(end-start+1);
                sum -= nums[start];
                start += 1;
            }
        }

        if min_len == usize::MAX {
            0
        } else {
            min_len as i32
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test1(){
        
    }
}