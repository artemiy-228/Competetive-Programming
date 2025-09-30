use std::io::{self, Write};

macro_rules! read_num {
    () => {{
        let mut input = String::new();
        std::io::stdin().read_line(&mut input).unwrap();
        input.trim().parse::<i32>().unwrap()
    }};
}

// функция бинарного поиска одного человека
fn find_person(mut l: i32, mut r: i32) -> i32 {
    while l < r {
        let mid = (l + r) / 2;
        println!("? {} {}", l, mid);
        io::stdout().flush().unwrap();
        let count = read_num!();
        if count >= 1 {
            r = mid;
        } else {
            l = mid + 1;
        }
    }
    l
}

fn main() {
    let mut l = 0;
    let mut r = 1023;
    let mut left = true;

    let mut last_l = 0;
    let mut last_r = 0;

    while l < r {
        let mid = (l + r) / 2;
        println!("? {} {}", l, mid);
        io::stdout().flush().unwrap();
        let count = read_num!();

        if count == 2 {
            // оба человека в левом диапазоне
            last_l = find_person(l, mid);
            last_r = find_person(mid + 1, r);
            break;
        } else if count == 1 {
            // один человек в левом диапазоне, второй — в правом
            last_l = find_person(l, mid);
            last_r = find_person(mid + 1, r);
            break;
        } else {
            // людей нет в левом диапазоне
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
