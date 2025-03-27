impl Solution {
    pub fn letter_combinations(digits: String) -> Vec<String> {
        if digits.is_empty() {
            return vec![];
        }

        let digit_to_letters: std::collections::HashMap<char, &str> = [
            ('2', "abc"),
            ('3', "def"),
            ('4', "ghi"),
            ('5', "jkl"),
            ('6', "mno"),
            ('7', "pqrs"),
            ('8', "tuv"),
            ('9', "wxyz"),
        ]
        .iter()
        .cloned()
        .collect();

        let mut result = Vec::new();
        let mut current = String::new();
        Self::backtrack(&digits, &digit_to_letters, &mut current, &mut result);
        result
    }

    fn backtrack(
        digits: &str,
        digit_to_letters: &std::collections::HashMap<char, &str>,
        current: &mut String,
        result: &mut Vec<String>,
    ) {
        if digits.is_empty() {
            result.push(current.clone());
            return;
        }

        let first_digit = digits.chars().next().unwrap();
        let letters = digit_to_letters[&first_digit];

        for c in letters.chars() {
            current.push(c);
            Self::backtrack(&digits[1..], digit_to_letters, current, result);
            current.pop();
        }
    }
}

pub struct Solution;
