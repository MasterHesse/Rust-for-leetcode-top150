pub struct Solution;

impl Solution {
    pub fn convert(s: String, num_rows: i32) -> String {
        if num_rows == 1 || s.len() <= num_rows as usize {
            return s;
        }

        let mut rows: Vec<String> = vec![String::new(); num_rows as usize];
        let mut cur_row: i32 = 0;
        let mut direction: i32 = 1;

        for char in s.chars() {
            rows[cur_row as usize].push(char);
            if cur_row == num_rows - 1 {
                direction = -1;
                cur_row += direction
            } else if cur_row == 0 {
                direction = 1;
                cur_row += direction;
            } else {
                cur_row += direction;
            }
        }

        rows.concat()
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test1() {
        let s = String::from("PAYPALISHIRING");
        let num_rows = 3;
        let expect = String::from("PAHNAPLSIIGYIR");
        let ret = Solution::convert(s, num_rows);
        assert_eq!(ret, expect);
    }
    #[test]
    fn test2() {
        let s = String::from("PAYPALISHIRING");
        let num_rows = 4;
        let expect = String::from("PINALSIGYAHRPI");
        let ret = Solution::convert(s, num_rows);
        assert_eq!(ret, expect);
    }
    #[test]
    fn test3() {
        let s = String::from("A");
        let num_rows = 1;
        let expect = String::from("A");
        let ret = Solution::convert(s, num_rows);
        assert_eq!(ret, expect);
    }
}
