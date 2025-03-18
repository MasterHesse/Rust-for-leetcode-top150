use std::collections::{HashMap, VecDeque};

impl Solution {
    pub fn can_finish(num_courses: i32, prerequisites: Vec<Vec<i32>>) -> bool {
        let num_courses = num_courses as usize;
        let mut graph: HashMap<i32, Vec<i32>> = HashMap::new();
        let mut in_degree = vec![0; num_courses];

        // 1. Build Graph and in-degree table
        for edge in prerequisites {
            let (course, prerequisite) = (edge[0], edge[1]);
            graph
                .entry(prerequisite)
                .or_insert_with(Vec::new)
                .push(course);
            in_degree[course as usize] += 1;
        }

        // 2. Initialize queue:
        // Put all courses with in-degree == 0 in the queue
        let mut queue: VecDeque<i32> = VecDeque::new();
        for (course, &degree) in in_degree.iter().enumerate() {
            if degree == 0 {
                queue.push_back(course as i32);
            }
        }

        // 3. BFS
        let mut count = 0;
        while let Some(courses) = queue.pop_front() {
            count += 1;
            if let Some(neighbors) = graph.get(&courses) {
                for &neighbor in neighbors {
                    in_degree[neighbor as usize] -= 1;
                    if in_degree[neighbor as usize] == 0 {
                        queue.push_back(neighbor);
                    }
                }
            }
        }

        // 4. Judge
        count == num_courses
    }
}

pub struct Solution;
