impl Solution {
    pub fn num_islands(mut grid: Vec<Vec<char>>) -> i32 {
        let mut count: i32 = 0;
        let row_len = grid.len();
        if row_len == 0 {
            return 0;
        }
        let col_len = grid[0].len();

        for i in 0..row_len {
            for j in 0..col_len {
                if grid[i][j] == '1' {
                    Self::dfs(i, j, &mut grid, row_len, col_len);
                    count += 1;
                }
            }
        }
        count
    }

    fn dfs(i: usize, j: usize, grid: &mut Vec<Vec<char>>, row_len: usize, col_len: usize) {
        // 检查当前节点是否为 '1'
        if grid[i][j] != '1' {
            return;
        }

        // 将当前节点标记为已访问
        grid[i][j] = '0';

        // 递归检查上下左右四个方向
        if i + 1 < row_len {
            Self::dfs(i + 1, j, grid, row_len, col_len);
        }
        if i > 0 {
            Self::dfs(i - 1, j, grid, row_len, col_len);
        }
        if j + 1 < col_len {
            Self::dfs(i, j + 1, grid, row_len, col_len);
        }
        if j > 0 {
            Self::dfs(i, j - 1, grid, row_len, col_len);
        }
    }
}

pub struct Solution;
