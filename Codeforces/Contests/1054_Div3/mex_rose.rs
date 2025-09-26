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
            .map(|x| x.parse::<i64>().unwrap())
            .collect::<Vec<i64>>()
    }};
}

fn main() {
    let t: usize = read_num!();

    for _ in 0..t {
        let n_k: Vec<i64> = read_array!();
        let n: usize = n_k[0] as usize;
        let k: usize = n_k[1] as usize;
        let a: Vec<i64> = read_array!();

        let mut count = vec![0; k + 1];
        let mut prefix = vec![0; k + 1];

        for i in a.iter() {
            if *i as usize <= k {
                count[*i as usize] += 1;
            }
        }
        for i in 1usize..=k {
            prefix[i] = prefix[i - 1];
            if count[i - 1] == 0 {
                prefix[i] += 1;
            }
        }
        println!("{}", prefix[k].max(count[k]));
    }
}
