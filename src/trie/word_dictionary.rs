use std::collections::HashMap;

struct WordDictionary {
    root: TrieNode,
}

struct TrieNode {
    children: HashMap<char, TrieNode>,
    is_end: bool,
}

impl TrieNode {
    fn new() -> Self {
        TrieNode {
            children: HashMap::new(),
            is_end: false,
        }
    }
}
/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl WordDictionary {
    fn new() -> Self {
        WordDictionary {
            root: TrieNode::new(),
        }
    }

    fn add_word(&mut self, word: String) {
        let mut node = &mut self.root;
        for c in word.chars() {
            node = node.children.entry(c).or_insert(TrieNode::new());
        }
        node.is_end = true
    }

    fn search(&self, word: String) -> bool {
        return Self::search_helper(&self.root, word.chars().collect());
    }

    fn search_helper(node: &TrieNode, mut chars: Vec<char>) -> bool {
        if chars.is_empty() {
            return node.is_end;
        }

        let c = chars.remove(0);
        if c == '.' {
            for child in node.children.values() {
                if Self::search_helper(child, chars.clone()) {
                    return true;
                }
            }
            false
        } else {
            match node.children.get(&c) {
                Some(child) => Self::search_helper(child, chars),
                None => false,
            }
        }
    }
}
