use std::io;

macro_rules! read_num {
    () => {{
        let mut input = String::new();
        io::stdin().read_line(&mut input);
        input.trim().parse::<usize>().unwrap()
    }};
}

macro_rules! read_array {
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
    let t: usize = read_num!();
    let mut counter: Vec<i32> = vec![0; 3];
    let mut count: i32 = 0;
    for _ in 0usize..t {
        let n: usize = read_num!();
        let arr: Vec<i32> = read_array!();
        for &e in arr.iter() {
            counter[(e + 1) as usize] += 1;
        }
        count += counter[1];
        if counter[0] % 2 != 0 {
            count += 2;
        }
        println!("{}", count);
        count = 0;
        counter[0] = 0;
        counter[1] = 0;
        counter[2] = 0;
    }
}
