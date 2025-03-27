use std::collections::{HashMap, VecDeque};

impl Solution {
    fn snakes_and_ladders(board: Vec<Vec<i32>>) -> i32 {
        let n = board.len();
        let target = (n * n) as i32;

        // 将棋盘坐标转换为方格编号
        let get_label = |r: usize, c: usize| -> i32 {
            let row = n - 1 - r; // 从下往上数行
            if row % 2 == 0 {
                // 从左往右数
                (row * n + c + 1) as i32
            } else {
                // 从右往左数
                (row * n + (n - 1 - c) + 1) as i32
            }
        };

        // 将方格编号转换为棋盘坐标
        let get_position = |label: i32| -> (usize, usize) {
            let label = label - 1; // 转换为 0-based
            let r = n - 1 - (label as usize / n); // 从下往上数行
            let c = if (label as usize / n) % 2 == 0 {
                label as usize % n // 从左往右数
            } else {
                n - 1 - (label as usize % n) // 从右往左数
            };
            (r, c)
        };

        // BFS 初始化
        let mut queue = VecDeque::new();
        queue.push_back(1); // 起点是方格 1
        let mut visited = HashMap::new();
        visited.insert(1, 0); // 记录每个方格的最少移动次数

        // BFS 主循环
        while let Some(curr) = queue.pop_front() {
            if curr == target {
                return visited[&curr]; // 到达终点，返回最少移动次数
            }

            // 模拟掷骰子，最多 6 步
            for next in (curr + 1)..=(curr + 6).min(target) {
                let (r, c) = get_position(next);
                let next_label = if board[r][c] != -1 {
                    board[r][c] // 如果有蛇或梯子，跳转到目标方格
                } else {
                    next // 否则，直接移动到 next
                };

                // 如果目标方格未被访问过，加入队列
                if !visited.contains_key(&next_label) {
                    visited.insert(next_label, visited[&curr] + 1);
                    queue.push_back(next_label);
                }
            }
        }

        -1 // 无法到达终点
    }
}

pub struct Solution;
