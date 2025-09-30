use std::io::{self, Write};

macro_rules! read_num {
    () => {{
        let mut input = String::new();
        std::io::stdin().read_line(&mut input).unwrap();
        input.trim().parse::<i32>().unwrap()
    }};
}

fn main() {
    let mut l = 0;
    let mut r = 1023;

    let mut last_l = 0;
    let mut last_r = 1023;
    let mut left: bool = true;

    while l < r {
        let mid = (r + l) / 2;
        println!("? {} {}", l, mid);
        io::stdout().flush().unwrap();
        let count = read_num!();
        if count == 2 {
            last_l = l;
            last_r = r;
            l = mid + 1;
        } else if count == 1 {
            let mut l1 = l;
            let mut r1 = mid;
            while l1 < r1 {
                let mid1 = (l1 + r1) / 2;
                println!("? {} {}", l1, mid1);
                io::stdout().flush().unwrap();
                let c1 = read_num!();
                if c1 == 1 {
                    r1 = mid1;
                } else {
                    l1 = mid1 + 1;
                }
            }
            last_l = l1;

            let mut l2 = mid + 1;
            let mut r2 = r;
            while l2 < r2 {
                let mid2 = (l2 + r2) / 2;
                println!("? {} {}", l2, mid2);
                io::stdout().flush().unwrap();
                let c2 = read_num!();
                if c2 == 1 {
                    r2 = mid2;
                } else {
                    l2 = mid2 + 1;
                }
            }
            last_r = l2;
            break;
        } else {
            if left {
                l = mid + 1;
                left = false;
            } else {
                r = mid;
                left = true;
            }
        }
    }
    println!("! {} {}", last_l, last_r);
    io::stdout().flush().unwrap();
}
