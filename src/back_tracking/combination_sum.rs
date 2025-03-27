impl Solution {
    pub fn combination_sum(candidates: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
        let mut res = Vec::new();
        let mut path = Vec::new();
        let mut sorted_candidates = candidates;
        sorted_candidates.sort();

        Self::backtrace(&sorted_candidates, target, 0, &mut path, &mut res);
        res
    }

    fn backtrace(
        candidates: &[i32],
        remaining: i32,
        start: usize,
        path: &mut Vec<i32>,
        res: &mut Vec<Vec<i32>>,
    ) {
        if remaining == 0 {
            res.push(path.clone());
            return;
        }

        for i in start..candidates.len() {
            let num = candidates[i];
            if num > remaining {
                break;
            }

            path.push(num);
            Self::backtrace(candidates, remaining - num, i, path, res);
            path.pop();
        }
    }
}

pub struct Solution;
