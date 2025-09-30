use std::io::{self, BufRead};

macro_rules! read_number {
    ($lines:expr) => {{
        $lines
            .next()
            .unwrap()
            .unwrap()
            .trim()
            .parse::<i128>()
            .unwrap()
    }};
}

macro_rules! read_array {
    ($lines:expr) => {{
        $lines
            .next()
            .unwrap()
            .unwrap()
            .trim()
            .split_whitespace()
            .map(|x| x.parse::<i128>().unwrap())
            .collect::<Vec<i128>>()
    }};
}

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    let n = read_number!(lines) as usize;
    let answers = read_array!(lines);

    let mut ans: i128 = -1;
    let mut counter: Vec<i128> = vec![0; n + 1];
    for i in answers.iter() {
        counter[*i as usize] += 1;
    }

    for l in 0..=n {
        let temp: i128 = n as i128 - l as i128;
        if counter[l] == temp {
            if ans == -1 || temp < ans {
                ans = temp;
            }
        }
    }

    println!("{}", ans);
}
