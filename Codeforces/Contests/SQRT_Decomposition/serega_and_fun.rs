use std::collections::HashMap;
use std::io;

#[derive(Debug, Clone)]
struct Component {
    pub size: usize,
    pub freq: HashMap<i32, i32>,
    pub array: Vec<i32>,
}

impl Component {
    pub fn new() -> Self {
        Self {
            size: 0,
            freq: HashMap::new(),
            array: Vec::new(),
        }
    }
}

macro_rules! input_num {
    () => {{
        let mut input = String::new();
        io::stdin().read_line(&mut input);
        input.trim().parse::<i32>().unwrap()
    }};
}

macro_rules! input_array {
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

pub fn build(decompose_arr: &mut Vec<Component>, array: &Vec<i32>, decompose_size: usize) {
    let n = array.len();

    for i in 0..decompose_arr.len() {
        for j in 0..decompose_size {
            let index = i * decompose_size + j;
            if index >= n {
                break;
            }
            let temp = array[index];
            decompose_arr[i].array.push(temp);
            *decompose_arr[i].freq.entry(temp).or_insert(0) += 1;
            decompose_arr[i].size += 1;
        }
    }
}

pub fn rebuild_decompose(decompose_arr: &mut Vec<Component>, decompose_size: usize) {
    let mut temp: Vec<i32> = Vec::new();

    for component in decompose_arr.iter_mut() {
        for e in component.array.iter() {
            temp.push(*e);
        }
        component.array.clear();
        component.freq.clear();
        component.size = 0;
    }
    build(decompose_arr, &temp, decompose_size);
}

fn find_block(decompose_arr: &Vec<Component>, index: usize) -> (usize, usize) {
    let mut sum: usize = 0;
    let mut a: usize = 0;
    let mut b: usize = 0;
    for (block_idx, block) in decompose_arr.iter().enumerate() {
        if index < sum + block.size {
            a = block_idx;
            b = index - sum;
            break;
        }
        sum += block.array.len();
    }
    return (a, b);
}

pub fn cycle_shift(
    decompose_arr: &mut Vec<Component>,
    l: usize,
    r: usize,
    rebuild: &mut bool,
    block_size: usize,
) {
    let (block_l, i_l) = find_block(&decompose_arr, l);
    let (block_r, j_r) = find_block(&decompose_arr, r);

    if block_l == block_r {
        let value = decompose_arr[block_l].array.remove(j_r);
        decompose_arr[block_l].array.insert(i_l, value);
    } else {
        let temp = decompose_arr[block_r].array.remove(j_r);
        *decompose_arr[block_r].freq.entry(temp).or_insert(0) -= 1;
        decompose_arr[block_r].size -= 1;

        if decompose_arr[block_r].size <= block_size / 4 {
            *rebuild = true;
        }

        decompose_arr[block_l].array.insert(i_l, temp);
        *decompose_arr[block_l].freq.entry(temp).or_insert(0) += 1;
        decompose_arr[block_l].size += 1;

        if decompose_arr[block_l].size >= 4 * block_size {
            *rebuild = true;
        }
    }
}

pub fn get_count(decompose_arr: &Vec<Component>, l: usize, r: usize, k: i32) -> i32 {
    let (block_l, i_l) = find_block(decompose_arr, l);
    let (block_r, j_r) = find_block(decompose_arr, r);

    let mut count = 0;

    if block_l == block_r {
        for i in i_l..=j_r {
            if decompose_arr[block_l].array[i] == k {
                count += 1;
            }
        }
    } else {
        for i in i_l..decompose_arr[block_l].array.len() {
            if decompose_arr[block_l].array[i] == k {
                count += 1;
            }
        }

        for block in (block_l + 1)..block_r {
            count += *decompose_arr[block].freq.get(&k).unwrap_or(&0);
        }
        for i in 0..=j_r {
            if decompose_arr[block_r].array[i] == k {
                count += 1;
            }
        }
    }

    count
}

fn main() {
    let n: usize = input_num!() as usize;
    let array: Vec<i32> = input_array!();

    let sqrt_f = (n as f64).sqrt();
    let num_blocks = if sqrt_f.fract() == 0.0 {
        sqrt_f as usize
    } else {
        sqrt_f as usize + 1
    };
    let block_size = (n + num_blocks - 1) / num_blocks;

    let mut decompose_arr: Vec<Component> = vec![Component::new(); num_blocks];
    build(&mut decompose_arr, &array, block_size);

    let q: usize = input_num!() as usize;
    let mut lastans: i32 = 0;
    let mut rebuild = false;

    for _ in 0..q {
        let query: Vec<i32> = input_array!();
        if query[0] == 1 {
            let mut l = ((query[1] + lastans - 1) % (n as i32)) as usize;
            let mut r = ((query[2] + lastans - 1) % (n as i32)) as usize;

            if l > r {
                std::mem::swap(&mut l, &mut r);
            }

            cycle_shift(&mut decompose_arr, l, r, &mut rebuild, block_size);
            if rebuild {
                rebuild_decompose(&mut decompose_arr, num_blocks);
                rebuild = false;
            }
        } else {
            let mut l = ((query[1] + lastans - 1) % (n as i32)) as usize;
            let mut r = ((query[2] + lastans - 1) % (n as i32)) as usize;
            let k = ((query[3] + lastans - 1) % (n as i32)) as i32 + 1;

            if l > r {
                std::mem::swap(&mut l, &mut r);
            }

            let ans = get_count(&mut decompose_arr, l, r, k);
            println!("{}", ans);
            lastans = ans;
        }
    }
}
