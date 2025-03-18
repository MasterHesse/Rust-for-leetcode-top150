pub struct Solution;

impl Solution {
    pub fn longest_common_prefix(strs: Vec<String>) -> String {
        if strs.is_empty() {
            return String::new();
        }

        let min_len = strs.iter().map(|s| s.len()).min().unwrap_or(0);

        let mut low = 0;
        let mut high = min_len;

        while low <= high {
            let mid = low + (high - low) / 2;
            if is_common_prefix(&strs, mid) {
                low = mid + 1;
            } else {
                high = mid - 1;
            }
        }

        strs[0][0..high].to_string()
    }
}

fn is_common_prefix(strs: &Vec<String>, len: usize) -> bool {
    let prefix = &strs[0][0..len];
    strs.iter().all(|s| s.starts_with(prefix))
}

#[cfg(test)]
mod test {

    use super::Solution;

    #[test]
    fn test1() {
        let strs: Vec<String> = vec![
            "flower".to_string(),
            "flow".to_string(),
            "flight".to_string(),
        ];
        let expect = String::from("fl");
        let result = Solution::longest_common_prefix(strs);
        assert_eq!(expect, result);
    }

    #[test]
    fn test2() {
        let strs: Vec<String> = vec!["dog".to_string(), "racecar".to_string(), "car".to_string()];
        let expect = String::from("");
        let result = Solution::longest_common_prefix(strs);
        assert_eq!(expect, result);
    }
}
