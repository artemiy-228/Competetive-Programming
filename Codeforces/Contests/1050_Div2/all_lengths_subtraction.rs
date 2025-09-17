use std::io;

fn main() {
    let mut input = String::new();

    io::stdin().read_line(&mut input);

    let n: i32 = input.trim().parse().unwrap();

    for _ in 0..n {
        input.clear();

        io::stdin().read_line(&mut input);

        let t: usize = input.trim().parse().unwrap();

        input.clear();
        io::stdin().read_line(&mut input);

        let permutation: Vec<i32> = input
            .trim()
            .split_whitespace()
            .map(|x| x.parse::<i32>().unwrap())
            .collect();

        let mut d: Vec<i32> = vec![1; t];

        let mut last = d[0];
        let mut swaps = 0;
        for i in 1..t {
            if permutation[i] > permutation[i - 1] {
                d[i] = 1;
            } else {
                d[i] = 0;
            }
            if d[i] != last {
                swaps += 1;
            }
            last = d[i];
        }

        let answer = if swaps <= 1 { "YES" } else { "NO" };
        println!("{}", answer);
    }
}
