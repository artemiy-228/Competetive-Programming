use std::cmp::min;
use std::io;

macro_rules! input_chars {
    () => {{
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        input.trim().chars().collect::<Vec<char>>()
    }};
}

fn main() {
    let a: Vec<char> = input_chars!();
    let b: Vec<char> = input_chars!();

    let a_size: usize = a.len();
    let b_size: usize = b.len();

    let mut dp: Vec<Vec<i32>> = vec![vec![0; b_size + 1]; a_size + 1];

    for i in 1..=a_size {
        dp[i][0] = dp[i - 1][0] + 1;
    }
    for j in 1..=b_size {
        dp[0][j] = dp[0][j - 1] + 1;
    }

    for i in 1..=a_size {
        for j in 1..=b_size {
            if a[i - 1] == b[j - 1] {
                dp[i][j] = dp[i - 1][j - 1];
            } else {
                dp[i][j] = min(dp[i - 1][j], min(dp[i - 1][j - 1], dp[i][j - 1])) + 1;
            }

            if i > 1 && j > 1 && a[i - 1] == b[j - 2] && a[i - 2] == b[j - 1] {
                dp[i][j] = min(dp[i][j], dp[i - 2][j - 2] + 1);
            }
        }
    }

    println!("{}", dp[a_size][b_size]);
}
