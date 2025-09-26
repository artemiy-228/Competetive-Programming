use std::io;

macro_rules! read_num {
    () => {{
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        input.trim().parse::<usize>().unwrap()
    }};
}

macro_rules! read_array {
    () => {{
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        input
            .trim()
            .split_whitespace()
            .map(|x| x.parse::<usize>().unwrap())
            .collect::<Vec<usize>>()
    }};
}

fn main() {
    let t: usize = read_num!();

    for _ in 0..t {
        let n_m: Vec<usize> = read_array!();
        let n = n_m[0];
        let m = n_m[1];
        let mut sets: Vec<Vec<usize>> = Vec::new();
        let mut freq: Vec<usize> = vec![0; m];
        for _ in 0..n {
            let set: Vec<usize> = read_array!();
            for e in 1..set.len() {
                freq[set[e] - 1] += 1;
            }
            sets.push((&set[1..]).to_vec());
        }
        let mut count = 0;
        if freq.iter().all(|&x| x != 0) {
            count += 1;
        }
        if count == 0 {
            println!("NO");
            continue;
        }
        'set_loop: for set in sets.iter() {
            for &e in set.iter() {
                if freq[e as usize - 1] == 1 {
                    continue 'set_loop;
                }
            }
            count += 1;
        }

        let answer = if count >= 3 { "YES" } else { "NO" };
        println!("{}", answer);
    }
}
