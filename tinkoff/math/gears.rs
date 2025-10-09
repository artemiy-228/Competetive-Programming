use std::io;

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

pub fn gcd(mut x: i64, mut y: i64) -> i64 {
    while x > 0 && y > 0 {
        if x > y {
            x = x % y;
        } else {
            y = y % x;
        }
    }
    std::cmp::max(x, y)
}

fn main() {
    let a_b = read_array!();
    let a = a_b[0];
    let b = a_b[1];

    let c = gcd(a, b);
    println!("{}", a * b / c);
}
