use advent_of_code::*;
use itertools::Itertools;

fn main() {
    let input = get_input(2025, 3);

    let mut counter = 0;

    for line in input.lines() {
        let digits = line
            .chars()
            .map(|c| c.to_digit(10).unwrap() as i32)
            .collect::<Vec<_>>();

        counter +=digits.iter().tuple_combinations().map(|(a, b)| a * 10 + b)
            .max().unwrap();

    }

    println!("{}", counter);
}
