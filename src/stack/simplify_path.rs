impl Solution {
    pub fn simplify_path(path: String) -> String {
        let mut stack: Vec<&str> = Vec::new();

        for part in path.split('/') {
            match part {
                ".." => {
                    stack.pop();
                } // 返回上一级目录
                "." | "" => {}         // 忽略当前目录 `.` 和空字符串
                _ => stack.push(part), // 其他情况入栈
            }
        }

        // 组装结果路径
        format!("/{}", stack.join("/"))
    }
}

pub struct Solution;
