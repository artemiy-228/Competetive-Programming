use std::io;

macro_rules! read_array {
    () => {{
        let mut input = String::new();
        io::stdin().read_line(&mut input);
        input
            .trim()
            .split_whitespace()
            .map(|x| x.parse::<i128>().unwrap())
            .collect()
    }};
}

fn main() {
    let a_b_k: Vec<i128> = read_array!();
    let mut a = a_b_k[0];
    let mut b = a_b_k[1];
    let mut k = a_b_k[2];
    let mut answer = 0;
    if b % k == 0 && (a % k != 0 || a % k == 0) {
        answer += (b * a - b) + (b / k - 1) * a;
    } else if b % k != 0 && a % k == 0 {
        answer += (b * a - a) + (a / k - 1) * b;
    } else {
        answer += -1;
    }
    println!("{}", answer);
}
