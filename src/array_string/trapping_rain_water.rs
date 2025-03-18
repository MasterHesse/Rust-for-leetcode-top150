pub struct Solution;

impl Solution {
    pub fn trap(height: Vec<i32>) -> i32 {
        if height.len() < 3 {
            return 0; // 不能形成雨水
        }

        let mut left = 0;
        let mut right = height.len() - 1;
        let mut left_max = 0;
        let mut right_max = 0;
        let mut water = 0;

        while left <= right {
            if height[left] < height[right] {
                if height[left] >= left_max {
                    left_max = height[left];
                } else {
                    water += left_max - height[left];
                }
                left += 1;
            } else {
                if height[right] >= right_max {
                    right_max = height[right];
                } else {
                    water += right_max - height[right];
                }
                right -= 1;
            }
        }

        water
    }
}


#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test1() {
        let height = vec![0, 1, 0, 2, 1, 0, 1, 3, 2, 1, 2, 1];
        let answer = 6;
        let expect = Solution::trap(height);

        assert_eq!(expect, answer);
    }

    #[test]
    fn test2() {
        let height = vec![4, 2, 0, 3, 2, 5];
        let answer = 9;
        let expect = Solution::trap(height);

        assert_eq!(expect, answer);
    }
}
