use std::io;

macro_rules! read_array {
    () => {{
        let mut input = String::new();
        io::stdin().read_line(&mut input);
        input
            .trim()
            .split_whitespace()
            .map(|x| x.parse::<f64>().unwrap())
            .collect()
    }};
}

fn main() {
    let nums: Vec<f64> = read_array!();
    println!("{}", nums[0] / nums[1]);
}
