use std::io;

macro_rules! read_arr {
    () => {{
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        input
            .trim()
            .split_whitespace()
            .map(|x| x.parse::<i128>().unwrap())
            .collect()
    }};
}

fn main() {
    let x_y: Vec<i128> = read_arr!();
    let mut x = x_y[0];
    let mut y = x_y[1];
    let mut counter = 0;
    let mut temp = x;
    let mut impossible = false;
    let mut last = 0;

    loop {
        temp = temp - y;
        counter += 1;

        if temp <= 0 {
            break;
        }
        temp += (x - temp) / 2;
        if last == temp {
            impossible = true;
            break;
        }

        last = temp;
    }
    let answer = if impossible { -1 } else { counter };
    println!("{}", answer);
}
