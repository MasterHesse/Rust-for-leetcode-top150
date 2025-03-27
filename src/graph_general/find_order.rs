use std::collections::{HashMap, VecDeque};

impl Solution {
    pub fn find_order(num_courses: i32, prerequisites: Vec<Vec<i32>>) -> Vec<i32> {
        let num_courses = num_courses as usize;
        let mut graph: HashMap<i32, Vec<i32>> = HashMap::new();
        let mut in_degree: Vec<i32> = vec![0; num_courses];

        for edge in prerequisites {
            let (course, prerequisite) = (edge[0], edge[1]);
            graph.entry(prerequisite).or_insert(Vec::new()).push(course);
            in_degree[course as usize] += 1;
        }

        let mut queue: VecDeque<i32> = VecDeque::new();
        for course in 0..num_courses {
            if in_degree[course as usize] == 0 {
                queue.push_back(course as i32);
            }
        }

        let mut result: Vec<i32> = Vec::new();

        while let Some(course) = queue.pop_front() {
            result.push(course);
            if let Some(neighbors) = graph.get(&course) {
                for &neighbor in neighbors {
                    in_degree[neighbor as usize] -= 1;
                    if in_degree[neighbor as usize] == 0 {
                        queue.push_back(neighbor);
                    }
                }
            }
        }
        if result.len() == num_courses {
            result
        } else {
            Vec::new()
        }
    }
}

pub struct Solution;
