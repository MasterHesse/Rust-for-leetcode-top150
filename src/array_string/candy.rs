pub struct Solution;
impl Solution {
    pub fn candy(ratings: Vec<i32>) -> i32 {
        let n = ratings.len();
        let mut candies = vec![1; n]; // 初始化每个学生至少 1 个糖果

        // 从左到右扫描，如果当前评分大于前一个，则糖果数比前一个多 1
        for i in 1..n {
            if ratings[i] > ratings[i - 1] {
                candies[i] = candies[i - 1] + 1;
            }
        }

        // 从右到左扫描，如果当前评分大于后一个，则糖果数比后一个多 1
        for i in (0..n - 1).rev() {
            if ratings[i] > ratings[i + 1] {
                candies[i] = candies[i].max(candies[i + 1] + 1);
            }
        }

        // 计算总糖果数
        candies.iter().sum()
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn debug() {
        let ratings = vec![1, 0, 2];
        let answer = Solution::candy(ratings);
        print!("{answer}");
    }

    #[test]
    fn test1() {
        let ratings = vec![1, 0, 2];
        let answer = 5;
        let expect = Solution::candy(ratings);
        assert_eq!(expect, answer);
    }

    #[test]
    fn test2() {
        let ratings = vec![1, 2, 2];
        let answer = 4;
        let expect = Solution::candy(ratings);
        assert_eq!(expect, answer);
    }
}
