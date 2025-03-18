pub struct Solution;

impl Solution {
    pub fn set_zeroes(matrix: &mut Vec<Vec<i32>>) {
        let rows_len = matrix.len();
        if rows_len == 0 {
            return;
        }
        let cols_len = matrix[0].len();

        let mut rows = vec![false; rows_len];
        let mut cols = vec![false; cols_len];

        for i in 0..rows_len {
            for j in 0..cols_len {
                if matrix[i][j] == 0 {
                    rows[i] = true;
                    cols[j] = true;
                }
            }
        }

        for i in 0..rows_len {
            for j in 0..cols_len {
                if rows[i] || cols[j] {
                    matrix[i][j] = 0;
                }
            }
        }
    }
}
