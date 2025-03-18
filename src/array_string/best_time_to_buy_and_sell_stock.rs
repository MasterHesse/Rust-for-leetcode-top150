pub struct Solution;

impl Solution {
    pub fn max_profit(prices: Vec<i32>) -> i32 {
        let mut min_price = prices[0];
        let mut max_profit = 0;

        for price in prices.iter() {
            if min_price > *price {
                min_price = *price;
            }

            if *price - min_price > max_profit {
                max_profit = *price - min_price;
            }
        }

        max_profit
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test() {
        {
            let prices = vec![7, 1, 5, 3, 6, 4];
            let max = Solution::max_profit(prices);
            let expect = 5;
            assert_eq!(max, expect);
        }

        {
            let prices = vec![7, 6, 4, 3, 1];
            let max = Solution::max_profit(prices);
            let expect = 0;
            assert_eq!(max, expect);
        }
    }
}
