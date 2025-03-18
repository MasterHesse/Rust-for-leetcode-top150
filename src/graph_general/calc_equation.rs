use std::collections::{HashMap, VecDeque};

impl Solution {
    pub fn calc_equation(
        equations: Vec<Vec<String>>,
        values: Vec<f64>,
        queries: Vec<Vec<String>>,
    ) -> Vec<f64> {
        // Step 1: Build the graph
        let mut graph: HashMap<String, Vec<(String, f64)>> = HashMap::new();

        for (i, equation) in equations.iter().enumerate() {
            let a = &equation[0];
            let b = &equation[1];
            let value = values[i];

            graph
                .entry(a.clone())
                .or_insert_with(Vec::new)
                .push((b.clone(), value));
            graph
                .entry(b.clone())
                .or_insert_with(Vec::new)
                .push((a.clone(), 1.0 / value));
        }

        // Step 2: Process each query
        let mut results = Vec::new();

        for query in queries {
            let c = &query[0];
            let d = &query[1];

            if !graph.contains_key(c) || !graph.contains_key(d) {
                results.push(-1.0);
                continue;
            }

            if c == d {
                results.push(1.0);
                continue;
            }

            // BFS to find the path from c to d
            let mut visited = HashMap::new();
            let mut queue = VecDeque::new();
            queue.push_back((c.clone(), 1.0));
            visited.insert(c.clone(), 1.0);

            let mut found = false;
            while let Some((node, current_value)) = queue.pop_front() {
                if node == *d {
                    results.push(current_value);
                    found = true;
                    break;
                }

                if let Some(neighbors) = graph.get(&node) {
                    for (neighbor, value) in neighbors {
                        if !visited.contains_key(neighbor) {
                            visited.insert(neighbor.clone(), current_value * value);
                            queue.push_back((neighbor.clone(), current_value * value));
                        }
                    }
                }
            }

            if !found {
                results.push(-1.0);
            }
        }

        results
    }
}

pub struct Solution;
