use std::io;

#[derive(Debug, Clone)]
struct Component {
    pub size: i32,
    pub block: Vec<i32>,
    pub freq: Vec<i32>,
}

impl Component {
    pub fn new(n: i32) -> Self {
        Component {
            size: 0,
            block: Vec::new(),
            freq: vec![0; n as usize],
        }
    }
}

pub fn get_count(
    decompose_arr: &Vec<Component>,
    l: usize,
    r: usize,
    k: i32,
    component_size: usize,
) -> i32 {
    let mut count = 0;
    let block_l = l / component_size;
    let block_r = r / component_size;

    if block_l == block_r {
        if l % component_size == 0 && r % component_size == decompose_arr[block_l].block.len() - 1 {
            count += decompose_arr[block_l].freq[k as usize - 1];
        } else {
            for i in (l % component_size)..=(r % component_size) {
                if decompose_arr[block_l].block[i] == k {
                    count += 1;
                }
            }
        }
    } else {
        for i in (l % component_size)..decompose_arr[block_l].block.len() {
            if decompose_arr[block_l].block[i] == k {
                count += 1;
            }
        }

        for b in (block_l + 1)..block_r {
            count += decompose_arr[b].freq[k as usize - 1];
        }

        for i in 0..=(r % component_size) {
            if decompose_arr[block_r].block[i] == k {
                count += 1;
            }
        }
    }

    count
}

pub fn rebuild_composition(decompose_arr: &mut Vec<Component>, component_size: usize, n: usize) {
    let mut arr: Vec<i32> = Vec::new();
    for component in decompose_arr.iter_mut() {
        for e in component.block.iter() {
            arr.push(*e);
        }
        component.block.clear();
        component.freq.fill(0);
        component.size = 0;
    }

    for i in 0..n {
        let block_index = i / component_size;
        decompose_arr[block_index].block.push(arr[i]);
        decompose_arr[block_index].freq[arr[i] as usize - 1] += 1;
        decompose_arr[block_index].size += 1;
    }
}

pub fn cycle_shift(decompose_arr: &mut Vec<Component>, l: usize, r: usize, component_size: usize) {
    let block_l = l / component_size;
    let block_r = r / component_size;
    let mut pos_l = l % component_size;
    let mut pos_r = r % component_size;

    if block_l == block_r {
        let val = decompose_arr[block_l].block.remove(pos_r);
        decompose_arr[block_l].block.insert(pos_l, val);
    } else {
        let mut val = decompose_arr[block_l].block.pop().unwrap();
        decompose_arr[block_l].freq[val as usize - 1] -= 1;

        for b in (block_l + 1)..block_r {
            decompose_arr[b].block.insert(0, val);
            decompose_arr[b].freq[val as usize - 1] += 1;

            val = decompose_arr[b].block.pop().unwrap();
            decompose_arr[b].freq[val as usize - 1] -= 1;
        }

        decompose_arr[block_r].block.insert(0, val);
        decompose_arr[block_r].freq[val as usize - 1] += 1;

        val = decompose_arr[block_r].block.remove(pos_r + 1);
        decompose_arr[block_r].freq[val as usize - 1] -= 1;
        decompose_arr[block_l].block.insert(pos_l, val);
        decompose_arr[block_l].freq[val as usize - 1] += 1;
    }
}

fn main() {
    let mut input = String::new();

    io::stdin().read_line(&mut input).unwrap();
    let n: i32 = input.trim().parse().unwrap();

    input.clear();
    io::stdin().read_line(&mut input).unwrap();
    let arr: Vec<i32> = input
        .trim()
        .split_whitespace()
        .map(|x| x.parse::<i32>().unwrap())
        .collect();

    let component_size = (n as f64).sqrt() as usize + 1;
    let num_blocks = (n as usize + component_size - 1) / component_size;

    let mut decompose_arr: Vec<Component> = vec![Component::new(n); num_blocks];

    for i in 0..n as usize {
        let block_index = i / component_size;
        decompose_arr[block_index].block.push(arr[i]);
        decompose_arr[block_index].freq[arr[i] as usize - 1] += 1;
        decompose_arr[block_index].size += 1;
    }

    input.clear();
    io::stdin().read_line(&mut input).unwrap();
    let q: i32 = input.trim().parse().unwrap();

    let mut lastans = 0;

    for _ in 0..q {
        input.clear();
        io::stdin().read_line(&mut input);

        let task: Vec<i32> = input
            .trim()
            .split_whitespace()
            .map(|x| x.parse::<i32>().unwrap())
            .collect();

        if task[0] == 1 {
            let mut l: usize = task[1] as usize;
            let mut r: usize = task[2] as usize;
            if l > r {
                let temp = l;
                l = r;
                r = temp;
            }
            l = ((l as i32 + lastans - 1) % n) as usize;
            r = ((r as i32 + lastans - 1) % n) as usize;

            cycle_shift(&mut decompose_arr, l, r, component_size);
        } else {
            let mut l: usize = task[1] as usize;
            let mut r: usize = task[2] as usize;
            if l > r {
                let temp = l;
                l = r;
                r = temp;
            }
            l = ((l as i32 + lastans - 1) % n) as usize;
            r = ((r as i32 + lastans - 1) % n) as usize;

            let k = ((task[3] + lastans - 1) % n + 1) as i32;

            let count = get_count(&decompose_arr, l, r, k, component_size);
            println!("{}", count);
            lastans = count;
        }
        println!("{:?}", decompose_arr);
    }
}
