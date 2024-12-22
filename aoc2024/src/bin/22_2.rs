use std::iter::repeat_n;

use advent_of_code::*;

const MOD : int = 16777216;

fn next_secret_number(x : int) -> int {
    let x2 = ((64 * x) ^ x) % MOD;
    let x3 = ((x2 / 32) ^ x2) % MOD;
    ((x3 * 2048) ^ x3) % MOD
}

fn main() {
    let input = get_input(2024, 22);

    let mut vendors = Vec::new();

    for line in input.lines() {
        let x = line.parse::<int>().unwrap();

        let mut prices = vec![x];

        for _ in 0..2000 {
            let last = *prices.last().unwrap();
            prices.push(next_secret_number(last));
        }

        let prices = prices.iter().map(|x| *x % 10).collect::<Vec<int>>();

        let mut patterns = HashMap::new();

        for win in prices.windows(5) {
            let diffs = win.iter().skip(1).zip(win.iter()).map(|(a,b)| a - b).collect::<Vec<int>>();
            if !patterns.contains_key(&diffs) {
                patterns.insert(diffs, win[4]);
            }
        }
        vendors.push(patterns);
    }

    let combinations : Vec<Vec<int>> = repeat_n(-9..=9, 4).multi_cartesian_product().collect();

    let mut best = 0;

    for comb in combinations {
        let mut acc = 0;
        for vendor in &vendors {
            if let Some(banana) = vendor.get(&comb) {
                acc += banana;
            }
        }
        best = best.max(acc);
    }

    println!("{}", best);
    
}