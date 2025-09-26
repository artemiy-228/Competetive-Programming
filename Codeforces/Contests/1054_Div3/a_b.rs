use std::io;

macro_rules! read_num {
    () => {{
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        input.trim().parse::<usize>().unwrap()
    }};
}

macro_rules! read_line {
    () => {{
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        input.trim().to_string()
    }};
}

fn main() {
    let t = read_num!();

    for _ in 0..t {
        let n = read_num!();
        let s = read_line!();
        let s_chars: Vec<char> = s.chars().collect();

        let mut blocks_a = 0;
        let mut blocks_b = 0;
        let mut prev = ' ';

        for &c in &s_chars {
            if c != prev {
                if c == 'a' {
                    blocks_a += 1;
                } else {
                    blocks_b += 1;
                }
                prev = c;
            }
        }

        let result = if blocks_a == 0 || blocks_b == 0 {
            0
        } else {
            std::cmp::min(blocks_a, blocks_b) - 1
        };

        println!("{}", result);
    }
}
