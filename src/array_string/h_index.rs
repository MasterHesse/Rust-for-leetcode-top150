pub struct Solution;

impl Solution {
    pub fn h_index(citations: Vec<i32>) -> i32 {
        let mut h = 0;
        loop {
            let mut count = 0;
            for element in citations.iter() {
                if *element >= h {
                    count += 1;
                }
            }

            if h > count {
                h -= 1;
                break;
            } else {
                h += 1;
            }
        }
        h
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test() {
        std::env::set_var("RUST_BACKTRACE", "full");

        {
            let citations = vec![3, 0, 6, 1, 5];
            let result = Solution::h_index(citations);
            let expect = 3;
            assert_eq!(result, expect);
        }

        {
            let citations = vec![1, 3, 1];
            let result = Solution::h_index(citations);
            let expect = 1;
            assert_eq!(result, expect);
        }
    }
}
