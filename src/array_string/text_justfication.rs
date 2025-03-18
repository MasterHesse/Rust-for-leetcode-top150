pub struct Solution;

impl Solution {
    pub fn full_justify(words: Vec<String>, max_width: i32) -> Vec<String> {
        let mut result = Vec::new();
        let mut current_line = Vec::new();
        let mut current_length = 0;
        let max_width = max_width as usize;

        for word in words {
            if current_length + word.len() + current_line.len() > max_width {
                // 分配空格并添加到结果中
                result.push(Self::justify_line(&current_line, current_length, max_width, false));
                current_line.clear();
                current_length = 0;
            }
            current_line.push(word.clone());
            current_length += word.len();
        }
        
        // 处理最后一行
        result.push(Self::justify_line(&current_line, current_length, max_width, true));
        result
    }

    fn justify_line(line: &Vec<String>, line_length: usize, max_width: usize, is_last_line: bool) -> String {
        if line.len() == 1 || is_last_line {
            // 最后一行或只有一个单词时，左对齐
            let mut result = line.join(" ");
            result.push_str(&" ".repeat(max_width - result.len()));
            return result;
        }

        // 普通行：分配空格
        let total_spaces = max_width - line_length;
        let slots = line.len() - 1;
        let space_per_slot = total_spaces / slots;
        let extra_spaces = total_spaces % slots;

        let mut result = String::new();
        for (i, word) in line.iter().enumerate() {
            result.push_str(word);
            if i < slots {
                result.push_str(&" ".repeat(space_per_slot + if i < extra_spaces { 1 } else { 0 }));
            }
        }
        result
    }
}


#[cfg(test)]
mod test {
    use super::Solution;

    #[test]
    fn test1() {
        let words = vec![
            "This",
            "is",
            "an",
            "example",
            "of",
            "text",
            "justification.",
        ]
        .iter()
        .map(|&s| s.to_string())
        .collect();
        let max_width = 16;
        let answer = vec!["This    is    an", "example  of text", "justification.  "];
        let result = Solution::full_justify(words, max_width);
        assert_eq!(result, answer);
    }

    #[test]
    fn test2() {
        let words = vec!["What", "must", "be", "acknowledgment", "shall", "be"]
            .iter()
            .map(|&s| s.to_string())
            .collect();
        let max_width = 16;
        let answer = vec!["What   must   be", "acknowledgment  ", "shall be        "];
        let result = Solution::full_justify(words, max_width);
        assert_eq!(result, answer);
    }

    #[test]
    fn test3() {
        let words = vec![
            "Science",
            "is",
            "what",
            "we",
            "understand",
            "well",
            "enough",
            "to",
            "explain",
            "to",
            "a",
            "computer.",
            "Art",
            "is",
            "everything",
            "else",
            "we",
            "do",
        ]
        .iter()
        .map(|&s| s.to_string())
        .collect();
        let max_width = 20;
        let answer = vec![
            "Science  is  what we",
            "understand      well",
            "enough to explain to",
            "a  computer.  Art is",
            "everything  else  we",
            "do                  ",
        ];
        let result = Solution::full_justify(words, max_width);
        assert_eq!(result, answer);
    }
}
