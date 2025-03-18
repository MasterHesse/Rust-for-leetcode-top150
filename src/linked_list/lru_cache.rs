use std::collections::{HashMap, VecDeque};

struct LRUCache {
    capacity: usize,        // 缓存容量
    map: HashMap<i32, i32>, // key -> value 映射
    usage: VecDeque<i32>,   // 记录 key 的使用顺序
}

impl LRUCache {
    fn new(capacity: i32) -> Self {
        LRUCache {
            capacity: capacity as usize,
            map: HashMap::new(),
            usage: VecDeque::new(),
        }
    }

    fn get(&mut self, key: i32) -> i32 {
        if let Some(&value) = self.map.get(&key) {
            self.update_usage(key);
            return value;
        }
        -1
    }

    fn put(&mut self, key: i32, value: i32) {
        if self.map.contains_key(&key) {
            self.update_usage(key);
        } else {
            if self.map.len() == self.capacity {
                if let Some(lru_key) = self.usage.pop_back() {
                    self.map.remove(&lru_key);
                }
            }
            self.usage.push_front(key);
        }
        self.map.insert(key, value);
    }

    fn update_usage(&mut self, key: i32) {
        self.usage.retain(|&k| k != key);
        self.usage.push_front(key);
    }
}
