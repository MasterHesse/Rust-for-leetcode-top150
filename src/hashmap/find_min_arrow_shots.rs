impl Solution {
    pub fn find_min_arrow_shots(mut points: Vec<Vec<i32>>) -> i32 {
        if points.is_empty() {
            return 0;
        }

        // 按右边界 `x_end` 进行升序排序
        points.sort_by_key(|balloon| balloon[1]);

        let mut arrows = 1; // 至少需要一支箭
        let mut end = points[0][1]; // 记录当前箭可以覆盖的最右端

        for balloon in points.iter().skip(1) {
            // 如果当前气球的 `x_start` 超出了 `end`，需要额外射一支箭
            if balloon[0] > end {
                arrows += 1;
                end = balloon[1]; // 这支新箭的射击位置
            }
        }

        arrows
    }
}

pub struct Solution;
