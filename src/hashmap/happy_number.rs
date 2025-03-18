use std::collections::HashMap;

impl Solution {
    pub fn is_happy(n: i32) -> bool {
        let mut seen: HashMap<i32, i32> = HashMap::new();
        let mut num = n;

        while num != 1 {
            if seen.contains_key(&num) {
                return false;
            }
            seen.insert(num, 1);
            num = Self::sum_of_square(num);
        }

        true
    }

    fn sum_of_square(mut n: i32) -> i32 {
        let mut sum = 0;
        while n > 0 {
            let digit = n % 10;
            sum += digit * digit;
            n /= 10;
        }
        sum
    }
}

pub struct Solution;
