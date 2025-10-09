use std::io;

macro_rules! input_num {
    () => {{
        let mut input = String::new();
        io::stdin().read_line(&mut input);
        input.trim().parse::<i32>().unwrap()
    }};
}
