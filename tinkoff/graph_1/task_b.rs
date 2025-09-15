use std::io;

pub fn dfs(graph: &Vec<Vec<i32>>, visited: &mut Vec<i32>, vertex: i32, is_cycle: &mut bool) {
    if *is_cycle {
        return;
    }
    visited[vertex as usize - 1] = 1;

    for v in graph[vertex as usize - 1].iter() {
        if visited[*v as usize - 1] == 0 {
            dfs(graph, visited, *v, is_cycle);
        } else if visited[*v as usize - 1] == 1 {
            *is_cycle = true;
            return;
        }
    }
    visited[vertex as usize - 1] = 2;
}

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input);
    let nums: Vec<i32> = input
        .trim()
        .split_whitespace()
        .map(|x| x.parse::<i32>().unwrap())
        .collect();

    let n: usize = nums[0] as usize;
    let m = nums[1];
    let mut adjecenty_list: Vec<Vec<i32>> = vec![Vec::new(); n];

    for _ in 0..m {
        input.clear();
        std::io::stdin().read_line(&mut input).unwrap();

        let edge: Vec<i32> = input
            .trim()
            .split_whitespace()
            .map(|x| x.parse::<i32>().unwrap())
            .collect();

        let v1 = edge[0];
        let v2 = edge[1];
        adjecenty_list[v1 as usize - 1].push(v2);
    }

    let mut is_cycle = false;
    let mut visited: Vec<i32> = vec![0; n];
    for v in 0..n {
        dfs(&adjecenty_list, &mut visited, v as i32 + 1, &mut is_cycle);
    }

    let answer = if is_cycle { 1 } else { 0 };

    println!("{:?}", answer);
}
