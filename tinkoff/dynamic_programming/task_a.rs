use std::io;

macro_rules! read_int {
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
            .map(|x| x.parse::<i32>().unwrap())
            .collect::<Vec<i32>>()
    }};
}

fn main() {
    let n: usize = read_int!();
    let steps: Vec<i32> = read_array!();

    let mut dp: Vec<i32> = vec![0; n];
    if n == 1 {
        println!("{}", steps[0]);
        return;
    }
    dp[0] = steps[0];
    dp[1] = std::cmp::min(dp[0] + steps[1], steps[1]);

    for i in 2..n {
        dp[i] = std::cmp::min(dp[i - 1] + steps[i], dp[i - 2] + steps[i]);
    }

    println!("{}", dp[n - 1]);
}
