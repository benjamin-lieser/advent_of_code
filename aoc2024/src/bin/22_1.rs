use advent_of_code::*;

const MOD : int = 16777216;

fn next_secret_number(x : int) -> int {
    let x2 = ((64 * x) ^ x) % MOD;
    let x3 = ((x2 / 32) ^ x2) % MOD;
    ((x3 * 2048) ^ x3) % MOD

}

fn main() {
    let input = get_input(2024, 22);

    let mut acc = 0;

    for line in input.lines() {
        let x = line.parse::<int>().unwrap();

        let secret = apply_n_times(next_secret_number, 2000)(x);

        acc += secret;
    }

    println!("{}", acc);
    
}