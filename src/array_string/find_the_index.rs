pub struct Solution;

impl Solution {
    pub fn str_str(haystack: String, needle: String) -> i32 {
        let haystack_chars: Vec<char> = haystack.chars().collect();
        let needle_chars: Vec<char> = needle.chars().collect();

        if haystack_chars.len() < needle_chars.len() {
            return -1;
        }
        if needle_chars.len() == 0 {
            return 0;
        }

        for i in 0..haystack_chars.len() {
            if i + needle_chars.len() > haystack_chars.len() {
                return -1;
            }
            if haystack_chars[i] == needle_chars[0] {
                let mut temp = 1;
                for j in 1..needle_chars.len() {
                    if haystack_chars[i + j] == needle_chars[j] {
                        temp += 1;
                    } else {
                        break;
                    }
                }
                if temp == needle_chars.len() {
                    return i as i32;
                }
            }
        }

        -1
    }
}

#[cfg(test)]
mod test {
    use super::Solution;

    #[test]
    fn test1() {
        let haystack = String::from("sadbutsad");
        let needle = String::from("sad");
        let answer: i32 = 0;
        let result = Solution::str_str(haystack, needle);
        assert_eq!(result, answer);
    }

    #[test]
    fn test2() {
        let haystack = String::from("leetcode");
        let needle = String::from("leetto");
        let answer: i32 = -1;
        let result = Solution::str_str(haystack, needle);
        assert_eq!(result, answer);
    }
}
