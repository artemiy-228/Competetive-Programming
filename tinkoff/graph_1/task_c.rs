use std::io;

fn main() {
    let mut input = String::new();

    io::stdin().read_line(&mut input);

    let n_m: Vec<i32> = input
        .trim()
        .split_whitespace()
        .map(|x| x.parse::<i32>().unwrap())
        .collect();

    let mut n = n_m[0];
    let mut m = n_m[1];

    let mut edges: Vec<(i32, i32)> = Vec::new();

    for i in 0..m {
        input.clear();
        io::stdin().read_line(&mut input);

        let mut u_v: Vec<i32> = input
            .trim()
            .split_whitespace()
            .map(|x| x.parse::<i32>().unwrap())
            .collect();

        edges.push((u_v[0], u_v[1]));
    }
    input.clear();
    io::stdin().read_line(&mut input);
    let sequence: Vec<i32> = input
        .trim()
        .split_whitespace()
        .map(|x| x.parse::<i32>().unwrap())
        .collect();

    let mut pos = vec![0; n as usize + 1];
    for (i, v) in sequence.iter().enumerate() {
        pos[*v as usize] = i;
    }

    let mut flag = true;
    for (u, v) in edges.iter() {
        if pos[*u as usize] > pos[*v as usize] {
            flag = false;
            break;
        }
    }

    let answer = if flag { "YES" } else { "NO" };

    println!("{}", answer);
}
