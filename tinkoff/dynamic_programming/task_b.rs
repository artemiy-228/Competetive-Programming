use std::io;

macro_rules! read_int {
    () => {{
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        input.trim().parse::<usize>().unwrap()
    }};
}

fn main() {
    let n: usize = read_int!();

    let mut dp: Vec<Vec<i32>> = vec![vec![1; n]; 3];

    for i in 1..n {
        dp[0][i] = dp[1][i - 1] + dp[2][i - 1];
        dp[1][i] = dp[0][i - 1] + dp[1][i - 1] + dp[2][i - 1];
        dp[2][i] = dp[0][i - 1] + dp[1][i - 1] + dp[2][i - 1];
    }
    let answer: i32 = dp[0][n - 1] + dp[1][n - 1] + dp[2][n - 1];
    println!("{}", answer);
}
