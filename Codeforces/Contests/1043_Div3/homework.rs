use std::io;

macro_rules! read_str {
    () => {{
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        input.trim_end().to_string()
    }};
}

macro_rules! read_int {
    () => {{
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        input.trim().parse::<i32>().unwrap()
    }};
}

fn main() {
    let t: i32 = read_int!();

    for _ in 0..t {
        let n: i32 = read_int!();
        let mut a: String = read_str!();
        let m: usize = read_int!() as usize;
        let b: String = read_str!();
        let c: String = read_str!();

        let b_chars: Vec<char> = b[..m].chars().collect();
        let c_chars: Vec<char> = c[..m].chars().collect();

        for i in 0..m {
            let ch_b = b_chars[i];
            let ch_c = c_chars[i];

            if ch_c == 'D' {
                a.push(ch_b);
            } else {
                a.insert(0, ch_b);
            }
        }

        println!("{}", a);
    }
}
