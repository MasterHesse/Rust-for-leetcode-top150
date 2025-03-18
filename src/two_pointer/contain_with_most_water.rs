pub struct Solution;

impl Solution {
    pub fn max_area(height: Vec<i32>) -> i32 {
        let mut left: usize = 0;
        let mut right: usize = height.len() - 1;
        let mut area: i32 = 0;

        while left < right {
            let h = i32::min(height[left], height[right]);
            let width = (right - left) as i32;
            area = i32::max(area, h * width);

            if height[left] < height[right] {
                left += 1;
            } else {
                right -= 1;
            }
        }

        area
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test1() {
        let height = vec![1, 8, 6, 2, 5, 4, 8, 3, 7];
        let answer = 49;
        let expect = Solution::max_area(height);
        assert_eq!(answer, expect);
    }

    #[test]
    fn test2() {
        let height = vec![1, 1];
        let answer = 1;
        let expect = Solution::max_area(height);
        assert_eq!(answer, expect);
    }
}
