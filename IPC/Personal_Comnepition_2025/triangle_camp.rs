use std::io;

macro_rules! input_num {
    () => {{
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        input.trim().parse::<i32>().unwrap()
    }};
}

fn main() {
    let num: i32 = input_num!();
    let mut b = 2;
    let mut a = num;
    if a == 4 {
        b = 4;
        a = 2;
    }
    let center = a / 2;
    let correction: i32 = if a / 2 != 0 { 0 } else { 1 };
    println!("{} {}", 0 + correction, 0);
    println!("{} {}", a + correction, 0);
    println!("{} {}", center, b);
}
