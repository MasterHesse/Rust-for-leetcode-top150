use std::collections::HashSet;

// 定义 Trie 结构体
#[derive(Default)]
struct TrieNode {
    children: [Option<Box<TrieNode>>; 26],
    is_word: bool,
}

impl TrieNode {
    fn new() -> Self {
        Default::default()
    }
}

struct Trie {
    root: TrieNode,
}

impl Trie {
    fn new() -> Self {
        Trie {
            root: TrieNode::new(),
        }
    }

    fn insert(&mut self, word: &str) {
        let mut node = &mut self.root;
        for c in word.chars() {
            let idx = (c as u8 - b'a') as usize;
            node = node.children[idx].get_or_insert_with(|| Box::new(TrieNode::new()));
        }
        node.is_word = true;
    }

    fn search(&self, word: &str) -> bool {
        let mut node = &self.root;
        for c in word.chars() {
            let idx = (c as u8 - b'a') as usize;
            if let Some(child) = &node.children[idx] {
                node = child;
            } else {
                return false;
            }
        }
        node.is_word
    }

    fn starts_with(&self, prefix: &str) -> bool {
        let mut node = &self.root;
        for c in prefix.chars() {
            let idx = (c as u8 - b'a') as usize;
            if let Some(child) = &node.children[idx] {
                node = child;
            } else {
                return false;
            }
        }
        true
    }
}

impl Solution {
    pub fn find_words(board: Vec<Vec<char>>, words: Vec<String>) -> Vec<String> {
        let mut res: HashSet<String> = HashSet::new();
        let rows = board.len();
        if rows == 0 {
            return Vec::new();
        }
        let cols = board[0].len();

        // 构建 Trie
        let mut trie = Trie::new();
        for word in &words {
            trie.insert(word);
        }

        let mut visited = vec![vec![false; cols]; rows];

        for x in 0..rows {
            for y in 0..cols {
                let c = board[x][y];
                let prefix = c.to_string();
                if trie.starts_with(&prefix) {
                    let mut path = String::new();
                    Self::dfs(&board, x, y, &mut visited, &mut path, &mut trie, &mut res);
                }
            }
        }

        res.into_iter().collect()
    }

    fn dfs(
        board: &Vec<Vec<char>>,
        x: usize,
        y: usize,
        visited: &mut Vec<Vec<bool>>,
        path: &mut String,
        trie: &mut Trie,
        res: &mut HashSet<String>,
    ) {
        let rows = board.len();
        let cols = board[0].len();

        // 越界检查或已访问
        if x >= rows || y >= cols || visited[x][y] {
            return;
        }

        let c = board[x][y];
        path.push(c);
        visited[x][y] = true;

        // 检查当前路径是否是一个完整的单词
        if trie.search(&path) {
            res.insert(path.clone());
        }

        // 检查当前路径是否是某个单词的前缀
        if trie.starts_with(&path) {
            // 四个方向 DFS
            let directions = [(0, 1), (1, 0), (0, -1), (-1, 0)];
            for &(dx, dy) in &directions {
                let nx = x as i32 + dx;
                let ny = y as i32 + dy;
                if nx >= 0 && ny >= 0 {
                    let nx = nx as usize;
                    let ny = ny as usize;
                    Self::dfs(board, nx, ny, visited, path, trie, res);
                }
            }
        }

        // 回溯
        path.pop();
        visited[x][y] = false;
    }
}

pub struct Solution;
