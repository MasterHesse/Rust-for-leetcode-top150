impl Solution {
    pub fn summary_ranges(nums: Vec<i32>) -> Vec<String> {
        let mut result = Vec::new();
        let mut iter = nums.iter();

        if let Some(&first) = iter.next() {
            let mut start = first;
            let mut prev = first;

            for &num in iter {
                if num != prev + 1 {
                    if start == prev {
                        result.push(format!("{}", start));
                    } else {
                        result.push(format!("{}->{}", start, prev));
                    }
                    start = num; // ✅ 这里不会再报错，因为 `start` 是 `mut`
                }
                prev = num;
            }

            // 处理最后一个区间
            if start == prev {
                result.push(format!("{}", start));
            } else {
                result.push(format!("{}->{}", start, prev));
            }
        }

        result
    }
}

pub struct Solution;
