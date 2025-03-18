impl Solution {
    pub fn solve(board: &mut Vec<Vec<char>>) {
        if board.is_empty() {
            return;
        }

        let rows = board.len();
        let cols = board[0].len();

        // Step 1: Mark the 'O's on the border and connected to it
        for i in 0..rows {
            for j in 0..cols {
                if (i == 0 || i == rows - 1 || j == 0 || j == cols - 1) && board[i][j] == 'O' {
                    dfs(board, i, j);
                }
            }
        }

        // Step 2: Replace all unmarked 'O's with 'X'
        for i in 0..rows {
            for j in 0..cols {
                if board[i][j] == 'O' {
                    board[i][j] = 'X';
                }
            }
        }

        // Step 3: Restore the marked 'T's back to 'O'
        for i in 0..rows {
            for j in 0..cols {
                if board[i][j] == 'T' {
                    board[i][j] = 'O';
                }
            }
        }

        fn dfs(board: &mut Vec<Vec<char>>, i: usize, j: usize) {
            let rows = board.len();
            let cols = board[0].len();

            if i >= rows || j >= cols || board[i][j] != 'O' {
                return;
            }

            // Mark the current cell as 'T'
            board[i][j] = 'T';

            // Recursively visit the neighboring cells
            if i > 0 {
                dfs(board, i - 1, j);
            }
            if i < rows - 1 {
                dfs(board, i + 1, j);
            }
            if j > 0 {
                dfs(board, i, j - 1);
            }
            if j < cols - 1 {
                dfs(board, i, j + 1);
            }
        }
    }
}
pub struct Solution;
