pub struct Solution;

impl Solution {
    pub fn game_of_life(board: &mut Vec<Vec<i32>>) {
        let rows = board.len();
        if rows == 0 {
            return;
        }
        let cols = board[0].len();

        // Directions for 8 neighbors
        let directions = [
            (-1, -1),
            (-1, 0),
            (-1, 1),
            (0, -1),
            (0, 1),
            (1, -1),
            (1, 0),
            (1, 1),
        ];

        // First pass: determine the next state
        for i in 0..rows {
            for j in 0..cols {
                let mut live_neighbors = 0;

                // Count live neighbors
                for (dx, dy) in &directions {
                    let x = i as i32 + dx;
                    let y = j as i32 + dy;
                    if x >= 0 && x < rows as i32 && y >= 0 && y < cols as i32 {
                        if board[x as usize][y as usize] == 1 || board[x as usize][y as usize] == -1
                        {
                            live_neighbors += 1;
                        }
                    }
                }

                // Apply the rules to determine the next state
                if board[i][j] == 1 {
                    if live_neighbors < 2 || live_neighbors > 3 {
                        // Cell dies
                        board[i][j] = -1; // -1 represents a cell that was live and will die
                    }
                } else {
                    if live_neighbors == 3 {
                        // Cell becomes alive
                        board[i][j] = 2; // 2 represents a cell that was dead and will become alive
                    }
                }
            }
        }

        // Second pass: update the board to the next state
        for i in 0..rows {
            for j in 0..cols {
                if board[i][j] == -1 {
                    board[i][j] = 0;
                } else if board[i][j] == 2 {
                    board[i][j] = 1;
                }
            }
        }
    }
}
