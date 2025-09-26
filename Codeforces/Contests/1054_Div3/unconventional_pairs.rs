use std::io;

macro_rules! read_num {
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
            .map(|x| x.parse::<i64>().unwrap())
            .collect::<Vec<i64>>()
    }};
}

fn main() {
    let t: usize = read_num!();

    for _ in 0..t {
        let n: usize = read_num!();
        let mut array: Vec<i64> = read_array!();
        array.sort();

        let mut answer: i64 = 0;
        for i in (0..n).step_by(2) {
            let diff = (array[i + 1] - array[i]).abs();
            if diff > answer {
                answer = diff;
            }
        }

        println!("{}", answer);
    }
}
