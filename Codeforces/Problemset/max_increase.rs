use std::io;

macro_rules! input_num {
    () => {{
        let mut input = String::new();
        io::stdin().read_line(&mut input);
        input.trim().parse::<i32>().unwrap()
    }};
}

macro_rules! input_array {
    () => {{
        let mut input = String::new();
        io::stdin().read_line(&mut input);
        input
            .trim()
            .split_whitespace()
            .map(|x| x.parse::<i32>().unwrap())
            .collect()
    }};
}

fn main() {
    let n: usize = input_num!() as usize;
    let arr: Vec<i32> = input_array!();
    let mut dp = 1;
    let mut max = 1;
    for i in 1usize..n {
        if arr[i] > arr[i - 1] {
            dp = dp + 1;
        } else {
            dp = 1;
        }
        if dp > max {
            max = dp;
        }
    }
    println!("{}", max);
}
