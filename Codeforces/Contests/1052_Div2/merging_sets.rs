use std::io;

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

    for i in 0..t {
        let n_m: Vec<usize> = read_array!();
        let n = n_m[0];
        let m = n_m[1];
        let mut freq: Vec<usize> = vec![0; m];
        for j in 0..n {
            let sets: Vec<usize> = read_array!();
            for e in 1..sets.len() {
                freq[sets[e] - 1] += 1;
            }
        }
        let mut flag = true;
        use std::io::{self, Read};

        fn main() {
            let mut input = String::new();
            io::stdin().read_to_string(&mut input).unwrap();
            let mut iter = input.split_whitespace();

            let t: usize = iter.next().unwrap().parse().unwrap();

            for _ in 0..t {
                let n: usize = iter.next().unwrap().parse().unwrap();
                let m: usize = iter.next().unwrap().parse().unwrap();

                let mut belongs = vec![Vec::new(); m + 1];
                for set_id in 0..n {
                    let l: usize = iter.next().unwrap().parse().unwrap();
                    for _ in 0..l {
                        let x: usize = iter.next().unwrap().parse().unwrap();
                        belongs[x].push(set_id);
                    }
                }

                let mut critical_sets = vec![false; n];
                let mut unique_count = 0;
                for v in belongs.iter().skip(1) {
                    if v.len() == 1 {
                        let idx = v[0];
                        if !critical_sets[idx] {
                            critical_sets[idx] = true;
                            unique_count += 1;
                        }
                    }
                }

                let ans = if unique_count >= n - 1 { "NO" } else { "YES" };
                println!("{}", ans);
            }
        }

        for e in freq.iter() {
            if *e < 2 {
                flag = false;
            }
        }
        let answer = if flag { "YES" } else { "NO" };
        println!("{}", answer)
    }
}
