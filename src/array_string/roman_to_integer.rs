pub struct Solution;

use std::collections::HashMap;
impl Solution {
    pub fn roman_to_int(s: String) -> i32 {
        let mut map: HashMap<char, i32> = HashMap::new();
        map.insert('I', 1);
        map.insert('V', 5);
        map.insert('X', 10);
        map.insert('L', 50);
        map.insert('C', 100);
        map.insert('D', 500);
        map.insert('M', 1000);

        let char_vec: Vec<char> = s.chars().collect();
        let mut answer: i32 = 0;
        let mut i = 0;

        while i < char_vec.len() {
            let value = map[&char_vec[i]];

            if i + 1 < char_vec.len() {
                let next_value = map[&char_vec[i + 1]];
                if value < next_value {
                    answer += next_value - value;
                    i += 2;
                    continue;
                }
            }

            answer += value;
            i += 1;
        }

        answer
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test1() {
        let s = String::from("III");
        let answer: i32 = 3;
        let expect = Solution::roman_to_int(s);
        assert_eq!(expect, answer);
    }

    #[test]
    fn test2() {
        let s = String::from("LVIII");
        let answer: i32 = 58;
        let expect = Solution::roman_to_int(s);
        assert_eq!(expect, answer);
    }

    #[test]
    fn test3() {
        let s = String::from("MCMXCIV");
        let answer: i32 = 1994;
        let expect = Solution::roman_to_int(s);
        assert_eq!(expect, answer);
    }
}
