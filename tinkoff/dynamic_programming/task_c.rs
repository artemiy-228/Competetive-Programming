use std::io;

macro_rules! read_array {
    () => {{
        let mut input = String::new();

        io::stdin().read_line(&mut input).unwrap();
        input
            .trim()
            .split_whitespace()
            .map(|x| x.parse::<i32>().unwrap())
            .collect()
    }};
}

fn main() {
    let n_m: Vec<i32> = read_array!();
    let n = n_m[0] as usize;
    let k = n_m[1] as usize;

    let mut scores: Vec<i32> = read_array!();
    scores.insert(0, 0);
    scores.push(0);

    let mut dp = vec![i32::MIN; n];
    let mut from = vec![0; n + 1];

    dp[0] = 0;

    for i in 1usize..n {
        let mut max_prev: usize = i.saturating_sub(k as usize);
        let mut best = dp[max_prev];
        from[i] = max_prev;

        for j in (i.saturating_sub(k))..i {
            if dp[j as usize] > best {
                best = dp[j];
                from[i] = j;
            }
        }
        dp[i] = best + scores[i];
    }
    println!("{:?}", scores);

    println!("{:?}", dp);
}
