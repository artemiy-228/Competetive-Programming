use std::collections::VecDeque;
use std::io;

macro_rules! read_array_u {
    () => {{
        let mut input = String::new();
        io::stdin().read_line(&mut input);
        input
            .trim()
            .split_whitespace()
            .map(|x| x.parse::<usize>().unwrap())
            .collect()
    }};
}

macro_rules! read_array_int {
    () => {{
        let mut input = String::new();
        io::stdin().read_line(&mut input);
        input
            .trim()
            .split_whitespace()
            .map(|x| x.parse::<i32>().unwrap())
            .collect()
    }};
}

fn bfs(graph: &Vec<Vec<i32>>, visited: &mut Vec<(i32, i32)>, q: &mut VecDeque<i32>) {
    while let Some(&v) = q.pop_front() {
        for &i in graph[v].iter() {
            visited[i].0 += 1;
            if visited[i].1 == -1 && visited[i].0 * 2 >= graph[i].len() {
                q.push_back(i);
                visited[i].1 = visited[v].1 + 1;
            }
        }
    }
}

pub fn main() {
    let n_m_k: vec
}
