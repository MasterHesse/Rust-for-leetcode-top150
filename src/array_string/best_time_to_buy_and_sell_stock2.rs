pub struct Solution;

impl Solution {
    pub fn max_profit(prices: Vec<i32>) -> i32 {
        let mut max_profit = 0;
        let mut tmp_max_profit = 0;
        let mut min_price = prices[0];

        for price in prices {
            if min_price > price {
                min_price = price;
            }

            if tmp_max_profit < price - min_price {
                tmp_max_profit = price - min_price;
            } else {
                max_profit = tmp_max_profit + max_profit;
                tmp_max_profit = 0;
                min_price = price;
            }
        }
        max_profit = tmp_max_profit + max_profit;
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
            let expect = 7;
            assert_eq!(max, expect);
        }

        {
            let prices = vec![1, 2, 3, 4, 5];
            let max = Solution::max_profit(prices);
            let expect = 4;
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
