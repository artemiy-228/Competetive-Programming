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
            .collect()
    }};
}
fn intersection(x1: i128, x2: i128, y1: i128, y2: i128) -> i128 {
    (x2.min(y2) - x1.max(y1) + 1).max(0)
}

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    let number = read_number!(lines);

    let mut colors: Vec<(i128, i128)> = Vec::with_capacity(number as usize);
    for _ in 0..number {
        let arr: Vec<i128> = read_array!(lines);
        colors.push((arr[0], arr[1]));
    }
    let answer = match number {
        1 => colors[0].1 - colors[0].0 + 1,
        2 => {
            let c1 = colors[0].1 - colors[0].0 + 1;
            let c2 = colors[1].1 - colors[1].0 + 1;
            let i = intersection(colors[0].0, colors[0].1, colors[1].0, colors[1].1);
            c1 * c2 - i
        }
        3 => {
            let c1 = colors[0].1 - colors[0].0 + 1;
            let c2 = colors[1].1 - colors[1].0 + 1;
            let c3 = colors[2].1 - colors[2].0 + 1;

            let i12 = intersection(colors[0].0, colors[0].1, colors[1].0, colors[1].1);
            let i23 = intersection(colors[1].0, colors[1].1, colors[2].0, colors[2].1);
            let i123 = intersection(
                colors[0].0.max(colors[1].0).max(colors[2].0),
                colors[0].1.min(colors[1].1).min(colors[2].1),
                colors[0].0.max(colors[1].0).max(colors[2].0),
                colors[0].1.min(colors[1].1).min(colors[2].1),
            );

            c1 * c2 * c3 - i12 * c3 - i23 * c1 + i123
        }
        _ => 0,
    };

    println!("{}", answer);
}
