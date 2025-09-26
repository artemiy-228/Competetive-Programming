use std::io;

#[derive(Debug, Clone)]
struct Component {
    array: Vec<i32>,
    steps: Vec<i32>,
    next_elem: Vec<usize>,
    start_index: usize,
}

impl Component {
    pub fn new() -> Self {
        Self {
            array: Vec::new(),
            steps: Vec::new(),
            next_elem: Vec::new(),
            start_index: 0,
        }
    }
}

macro_rules! read_array_i32 {
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

macro_rules! read_array_u {
    () => {{
        let mut input = String::new();
        io::stdin().read_line(&mut input);
        input
            .trim()
            .split_whitespace()
            .map(|x| x.parse::<usize>().unwrap())
            .collect()
    }};
}

pub fn build(sq_dec: &mut Vec<Component>, power: &Vec<i32>, block_size: usize) {
    let power_len: usize = power.len();
    for i in 0usize..block_size {
        for j in 0usize..block_size {
            let index: usize = i * block_size + j;
            if index < power_len {
                sq_dec[i].array.push(power[index]);
                sq_dec[i].steps.push(0);
                sq_dec[i].next_elem.push(0);
            }
        }
        sq_dec[i].start_index = i * block_size;
        for j in (0..sq_dec[i].array.len()).rev() {
            let index = j + sq_dec[i].array[j] as usize;

            if index + sq_dec[i].start_index >= power_len {
                sq_dec[i].steps[j] = 1;
                sq_dec[i].next_elem[j] = usize::MAX;
            } else if index >= sq_dec[i].array.len() {
                sq_dec[i].steps[j] = 1;
                sq_dec[i].next_elem[j] = sq_dec[i].start_index + index;
            } else {
                sq_dec[i].steps[j] = sq_dec[i].steps[index] + 1;
                sq_dec[i].next_elem[j] = sq_dec[i].next_elem[index];
            }
        }
    }
}

pub fn set_power(
    sq_dec: &mut Vec<Component>,
    index: usize,
    value: i32,
    block_size: usize,
    n: usize,
) {
    let block = index / block_size;
    let local = index % block_size;

    sq_dec[block].array[local] = value;

    for j in (0..sq_dec[block].array.len()).rev() {
        let global_index = sq_dec[block].start_index + j;
        let to = global_index + sq_dec[block].array[j] as usize;

        if to >= n {
            sq_dec[block].steps[j] = 1;
            sq_dec[block].next_elem[j] = usize::MAX;
        } else if to / block_size != block {
            sq_dec[block].steps[j] = 1;
            sq_dec[block].next_elem[j] = to;
        } else {
            let local_to = to - sq_dec[block].start_index;
            sq_dec[block].steps[j] = sq_dec[block].steps[local_to] + 1;
            sq_dec[block].next_elem[j] = sq_dec[block].next_elem[local_to];
        }
    }
}

pub fn throw_ball(sq_dec: &Vec<Component>, ball: usize, block_size: usize) -> (usize, usize) {
    let mut count: usize = 0;
    let mut curr: usize = ball;

    loop {
        let block = curr / block_size;
        let id = curr % block_size;

        count += sq_dec[block].steps[id] as usize;
        let next = sq_dec[block].next_elem[id];

        if next == usize::MAX {
            return (curr + 1, count);
        } else {
            curr = next;
        }
    }
}

fn main() {
    let n_m: Vec<usize> = read_array_u!();
    let n: usize = n_m[0];
    let m: usize = n_m[1];

    let power: Vec<i32> = read_array_i32!();

    let sqrt_f = (n as f64).sqrt();
    let num_blocks = if sqrt_f.fract() == 0.0 {
        sqrt_f as usize
    } else {
        sqrt_f as usize + 1
    };

    let mut sq_dec: Vec<Component> = vec![Component::new(); num_blocks];

    build(&mut sq_dec, &power, num_blocks);
    for _ in 0..m {
        let action: Vec<i32> = read_array_i32!();

        if action[0] == 0 {
            let a = (action[1] - 1) as usize;
            let b = action[2];
            set_power(&mut sq_dec, a, b, num_blocks, n);
        } else {
            let a = (action[1] - 1) as usize;
            let (last, steps) = throw_ball(&sq_dec, a, num_blocks);
            println!("{} {}", last, steps);
        }
    }
}
