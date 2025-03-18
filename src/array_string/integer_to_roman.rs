pub struct Solution;

impl Solution {
    pub fn int_to_roman(num: i32) -> String {
        let values = [1000, 900, 500, 400, 100, 90, 50, 40, 10, 9, 5, 4, 1];
        let symbols = [
            "M", "CM", "D", "CD", "C", "XC", "L", "XL", "X", "IX", "V", "IV", "I",
        ];

        let mut answer = String::new();
        let mut temp_num = num;

        for (i, &value) in values.iter().enumerate() {
            while temp_num >= value {
                answer.push_str(symbols[i]);
                temp_num -= value;
            }
        }

        answer
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test1() {
        let num = 3749;
        let answer = String::from("MMMDCCXLIX");
        let expect = Solution::int_to_roman(num);
        assert_eq!(expect, answer);
    }

    #[test]
    fn test2() {
        let num = 58;
        let answer = String::from("LVIII");
        let expect = Solution::int_to_roman(num);
        assert_eq!(expect, answer);
    }

    #[test]
    fn test3() {
        let num = 1994;
        let answer = String::from("MCMXCIV");
        let expect = Solution::int_to_roman(num);
        assert_eq!(expect, answer);
    }
}
