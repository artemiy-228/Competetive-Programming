use std::collections::HashMap;
use std::io::{self, BufRead};

macro_rules! read_num {
    () => {{
        let mut input = String::new();
        io::stdin().read_line(&mut input);
        input.trim().parse::<usize>().unwrap()
    }};
}

macro_rules! read_array {
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

fn main() {
    let t: usize = read_num!();

    for _ in 0..t {
        let n: usize = read_num!();
        let a: Vec<usize> = read_array!();

        let mut freq: HashMap<usize, usize> = HashMap::new();
        for x in &a {
            *freq.entry(*x).or_insert(0) += 1;
        }

        let counts: Vec<usize> = freq.values().cloned().collect();
        let counts: Vec<usize> = freq.values().cloned().collect();

        let mut max_count = 0;
        for &c in &counts {
            if c > max_count {
                max_count = c;
            }
        }

        let mut max_len = 0;
        for k in 1..=max_count {
            let mut num_elements = 0;
            for &c in &counts {
                if c >= k {
                    num_elements += 1;
                }
            }
            let len = num_elements * k;
            if len > max_len {
                max_len = len;
            }
        }

        println!("{}", max_len);
    }
}
