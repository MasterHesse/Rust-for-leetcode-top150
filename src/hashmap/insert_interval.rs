impl Solution {
    pub fn insert(intervals: Vec<Vec<i32>>, new_interval: Vec<i32>) -> Vec<Vec<i32>> {
        let mut result = Vec::new();
        let mut i = 0;
        let n = intervals.len();
        let (mut start, mut end) = (new_interval[0], new_interval[1]);

        while i < n && intervals[i][1] < start {
            result.push(intervals[i].clone());
            i += 1;
        }

        while i < n && intervals[i][0] <= end {
            start = start.min(intervals[i][0]);
            end = end.max(intervals[i][1]);
            i += 1;
        }
        result.push(vec![start, end]);

        while i < n {
            result.push(intervals[i].clone());
            i += 1;
        }

        result
    }
}

pub struct Solution;
