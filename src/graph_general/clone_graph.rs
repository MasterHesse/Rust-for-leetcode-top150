use std::collections::HashMap;

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Node {
    pub val: i32,
    pub neighbors: Vec<Option<Rc<RefCell<Node>>>>,
}

impl Node {
    #[inline]
    pub fn new(val: i32) -> Self {
        Node {
            val,
            neighbors: Vec::new(),
        }
    }
}

use std::cell::RefCell;
use std::rc::Rc;

pub fn clone_graph(node: Option<Rc<RefCell<Node>>>) -> Option<Rc<RefCell<Node>>> {
    if node.is_none() {
        return None;
    }

    let mut visited = HashMap::new(); // 记录原节点和新节点的映射关系
    dfs(node.clone(), &mut visited)
}

fn dfs(
    node: Option<Rc<RefCell<Node>>>,
    visited: &mut HashMap<i32, Rc<RefCell<Node>>>,
) -> Option<Rc<RefCell<Node>>> {
    if let Some(rc_node) = node {
        let node_ref = rc_node.borrow();
        let val = node_ref.val;

        // 如果节点已经访问过，直接返回新节点
        if let Some(new_node) = visited.get(&val) {
            return Some(new_node.clone());
        }

        // 创建新节点
        let new_node = Rc::new(RefCell::new(Node::new(val)));
        visited.insert(val, new_node.clone());

        // 递归克隆邻居节点
        for neighbor in &node_ref.neighbors {
            let cloned_neighbor = dfs(neighbor.clone(), visited);
            if let Some(cloned_neighbor) = cloned_neighbor {
                new_node.borrow_mut().neighbors.push(Some(cloned_neighbor));
            }
        }

        Some(new_node)
    } else {
        None
    }
}
