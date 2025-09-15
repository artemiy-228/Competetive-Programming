use std::io::{self, Write};

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let nums: Vec<i32> = input
        .trim()
        .split_whitespace()
        .map(|x| x.parse().unwrap())
        .collect();
    let n = nums[0];
    let m = nums[1];

    let mut graph: Vec<Vec<i32>> = vec![Vec::new(); n as usize];

    for _ in 0..m {
        input.clear();
        io::stdin().read_line(&mut input).unwrap();
        let edge: Vec<i32> = input
            .trim()
            .split_whitespace()
            .map(|x| x.parse().unwrap())
            .collect();
        let (v1, v2) = (edge[0], edge[1]);
        graph[v1 as usize - 1].push(v2);
        graph[v2 as usize - 1].push(v1);
    }

    let mut visited: Vec<bool> = vec![false; n as usize];
    let mut components: Vec<Vec<i32>> = Vec::new();

    for start in 1..=n {
        let idx = (start - 1) as usize;
        if visited[idx] {
            continue;
        }

        let mut stack = vec![start];
        let mut comp = Vec::new();
        visited[idx] = true;

        while let Some(vertex) = stack.pop() {
            comp.push(vertex);
            let v_idx = (vertex - 1) as usize;
            for &neighbor in &graph[v_idx] {
                let n_idx = (neighbor - 1) as usize;
                if !visited[n_idx] {
                    visited[n_idx] = true;
                    stack.push(neighbor);
                }
            }
        }

        comp.sort();
        components.push(comp);
    }

    let mut output = String::new();
    output.push_str(&format!("{}\n", components.len()));
    for comp in components {
        output.push_str(&format!("{}\n", comp.len()));
        for v in comp {
            output.push_str(&format!("{} ", v));
        }
        output.push('\n');
    }

    let mut stdout = io::BufWriter::new(io::stdout());
    stdout.write_all(output.as_bytes()).unwrap();
}
