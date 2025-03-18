pub struct Solution;

impl Solution {
    pub fn can_complete_circuit(gas: Vec<i32>, cost: Vec<i32>) -> i32 {
        let mut total_gas = 0;
        let mut total_cost = 0;
        let mut current_gas = 0;
        let mut start = 0;

        // 计算总油量和总消耗
        for i in 0..gas.len() {
            total_gas += gas[i];
            total_cost += cost[i];

            // 如果当前油量不足以继续，尝试从下一个加油站开始
            current_gas += gas[i] - cost[i];
            if current_gas < 0 {
                start = i + 1; // 重新设置起点
                current_gas = 0; // 重置油量
            }
        }

        // 如果总油量大于或等于总消耗，返回起点，否则返回 -1
        if total_gas >= total_cost {
            start as i32
        } else {
            -1
        }
    }
}

#[cfg(test)]
mod tests {

    use super::Solution;

    #[test]
    fn test1() {
        let gas = vec![1, 2, 3, 4, 5];
        let cost = vec![3, 4, 5, 1, 2];
        let answer = 3;
        let expect = Solution::can_complete_circuit(gas, cost);
        assert_eq!(expect, answer);
    }

    #[test]
    fn test2() {
        let gas = vec![2, 3, 4];
        let cost = vec![3, 4, 3];
        let answer = -1;
        let expect = Solution::can_complete_circuit(gas, cost);
        assert_eq!(expect, answer);
    }
}
