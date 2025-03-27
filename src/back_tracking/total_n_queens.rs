use std::collections::HashSet;

impl Solution {
    pub fn total_n_queens(n: i32) -> i32 {
        let mut count = 0;
        let mut cols: HashSet<i32> = HashSet::new();
        let mut diag1: HashSet<i32> = HashSet::new();
        let mut diag2: HashSet<i32> = HashSet::new();
        Self::backtrace(n, 0, &mut cols, &mut diag1, &mut diag2, &mut count);
        count
    }

    fn backtrace(
        n: i32,
        row: i32,
        cols: &mut HashSet<i32>,
        diag1: &mut HashSet<i32>,
        diag2: &mut HashSet<i32>,
        count: &mut i32,
    ) {
        if row == n {
            *count += 1;
            return;
        }

        for col in 0..n {
            let d1 = row - col;
            let d2 = row + col;

            if cols.contains(&col) || diag1.contains(&d1) || diag2.contains(&d2) {
                continue;
            }

            cols.insert(col);
            diag1.insert(d1);
            diag2.insert(d2);

            Self::backtrace(n, row + 1, cols, diag1, diag2, count);

            cols.remove(&col);
            diag1.remove(&d1);
            diag2.remove(&d2);
        }
    }
}

pub struct Solution;
