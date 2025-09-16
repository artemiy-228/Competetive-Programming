use std::collections::VecDeque;
use std::io;

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input);
    let mut n: i32 = input.trim().parse().unwrap();

    input.clear();
    io::stdin().read_line(&mut input);
    let mut x1_y1: Vec<i32> = input
        .trim()
        .split_whitespace()
        .map(|x| x.parse::<i32>().unwrap())
        .collect();
    let mut x1 = x1_y1[0];
    let mut y1 = x1_y1[1];

    input.clear();
    io::stdin().read_line(&mut input);
    let mut x2_y2: Vec<i32> = input
        .trim()
        .split_whitespace()
        .map(|x| x.parse::<i32>().unwrap())
        .collect();
    let mut x2 = x2_y2[0];
    let mut y2 = x2_y2[1];

    let mut map: Vec<Vec<i32>> = vec![vec![i32::MAX; n as usize + 1]; n as usize + 1];
    let mut visited: Vec<Vec<bool>> = vec![vec![false; n as usize + 1]; n as usize + 1];
    let mut from: Vec<Vec<(i32, i32)>> =
        vec![vec![(i32::MAX, i32::MAX); n as usize + 1]; n as usize + 1];

    let step_x = [2, 2, -2, -2, 1, -1, 1, -1];
    let step_y = [1, -1, 1, -1, 2, 2, -2, -2];

    let mut q: VecDeque<(i32, i32)> = VecDeque::from(vec![(x1, y1)]);
    map[x1 as usize][y1 as usize] = 0;
    visited[x1 as usize][y1 as usize] = true;
    from[x1 as usize][y1 as usize] = (-1, -1);
    while let Some((x, y)) = q.pop_front() {
        for i in 0..8 {
            let temp_x = x + step_x[i];
            let temp_y = y + step_y[i];

            if temp_x >= 1
                && temp_x <= n
                && temp_y >= 1
                && temp_y <= n
                && !visited[temp_x as usize][temp_y as usize]
            {
                visited[temp_x as usize][temp_y as usize] = true;
                map[temp_x as usize][temp_y as usize] = map[x as usize][y as usize] + 1;
                from[temp_x as usize][temp_y as usize] = (x, y);
                q.push_back((temp_x, temp_y));
            }
        }
    }
    while x2 != -1 && y2 != -1 {
        q.push_back((x2, y2));
        let (temp_x, temp_y) = from[x2 as usize][y2 as usize];
        x2 = temp_x;
        y2 = temp_y;
    }
    println!("{}", q.len() - 1);
    while let Some((x, y)) = q.pop_back() {
        println!("{} {}", x, y);
    }
}
