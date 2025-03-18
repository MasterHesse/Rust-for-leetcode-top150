pub struct Solution;

impl Solution {
    pub fn length_of_last_word(s: String) -> i32 {
        let mut count = 0;
        let mut has_started = false;

        for c in s.chars().rev() {
            if c == ' ' {
                if has_started {
                    break;
                }
            } else {
                has_started = true;
                count += 1;
            }
        }

        count
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test1() {
        let s: String = String::from("Hello World");
        let answer = 5;
        let expect = Solution::length_of_last_word(s);
        assert_eq!(expect, answer);
    }

    #[test]
    fn test2() {
        let s: String = String::from("   fly me   to   the moon  ");
        let answer = 4;
        let expect = Solution::length_of_last_word(s);
        assert_eq!(expect, answer);
    }

    #[test]
    fn test3() {
        let s: String = String::from("luffy is still joyboy");
        let answer = 6;
        let expect = Solution::length_of_last_word(s);
        assert_eq!(expect, answer);
    }
}
