use std::collections::HashMap;

struct RandomizedSet {
    map: HashMap<i32, usize>,
    list: Vec<i32>,
}

/*
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl RandomizedSet {
    pub fn new() -> Self {
        RandomizedSet {
            map: HashMap::new(),
            list: Vec::new(),
        }
    }

    fn insert(&mut self, val: i32) -> bool {
        if self.map.contains_key(&val) {
            return false;
        }
        self.list.push(val);
        self.map.insert(val, self.list.len() - 1);
        true
    }

    fn remove(&mut self, val: i32) -> bool {
        if let Some(&idx) = self.map.get(&val) {
            let last_element = self.list.pop().unwrap();
            if idx != self.list.len() {
                self.list[idx] = last_element;
                self.map.insert(last_element, idx);
            }
            self.map.remove(&val);
            return true;
        }
        false
    }

    fn get_random(&self) -> i32 {
        let rand_index = rand::random::<usize>() % self.list.len();
        self.list[rand_index]
    }
}

/*
 * Your RandomizedSet object will be instantiated and called as such:
 * let obj = RandomizedSet::new();
 * let ret_1: bool = obj.insert(val);
 * let ret_2: bool = obj.remove(val);
 * let ret_3: i32 = obj.get_random();
 */

#[cfg(test)]
mod tests {
    use super::RandomizedSet;
    use std::collections::HashSet;

    #[test]
    fn test_basic_operations() {
        let mut randomized_set = RandomizedSet::new();

        // 测试 insert
        assert_eq!(randomized_set.insert(1), true); // 插入成功
        assert_eq!(randomized_set.insert(2), true); // 插入成功
        assert_eq!(randomized_set.insert(1), false); // 重复插入，返回 false

        // 测试 remove
        assert_eq!(randomized_set.remove(2), true); // 删除成功
        assert_eq!(randomized_set.remove(3), false); // 元素不存在，返回 false
        assert_eq!(randomized_set.remove(2), false); // 元素已被删除，返回 false

        // 测试 getRandom
        assert_eq!(randomized_set.get_random(), 1); // 只剩下一个元素，getRandom 应返回 1
    }

    #[test]
    fn test_combined_operations() {
        let mut randomized_set = RandomizedSet::new();

        // 插入多个元素
        assert_eq!(randomized_set.insert(1), true);
        assert_eq!(randomized_set.insert(2), true);
        assert_eq!(randomized_set.insert(3), true);

        // 删除一个元素
        assert_eq!(randomized_set.remove(2), true);

        // 测试 getRandom 返回剩余的元素
        let mut result_set = HashSet::new();
        for _ in 0..100 {
            result_set.insert(randomized_set.get_random());
        }
        assert!(result_set.contains(&1)); // 1 应该仍在集合中
        assert!(result_set.contains(&3)); // 3 应该仍在集合中
        assert!(!result_set.contains(&2)); // 2 已被删除，不应该出现在结果中
    }

    #[test]
    fn test_randomness() {
        let mut randomized_set = RandomizedSet::new();

        // 插入多个元素
        randomized_set.insert(10);
        randomized_set.insert(20);
        randomized_set.insert(30);

        // 测试随机性
        let mut frequencies = vec![0; 3];
        let values = vec![10, 20, 30];
        for _ in 0..300 {
            let rand_val = randomized_set.get_random();
            if let Some(idx) = values.iter().position(|&v| v == rand_val) {
                frequencies[idx] += 1;
            }
        }

        // 检查随机性分布是否合理（以 ±20% 的偏差范围为合理）
        for freq in frequencies {
            assert!((80..120).contains(&freq)); // 300 次调用中每个值大约出现 100 次
        }
    }
}
