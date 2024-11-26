use advent_of_code::*;

fn main() {
    let input = get_input(2023, 1);
    
    let mut acc = 0;

    for line in input.lines() {
        let digits: Vec<u32> = line.chars().filter_map(|c| c.to_digit(10)).collect();
        let first = digits[0] as u64;
        let second = *digits.last().unwrap() as u64;
        acc += first * 10 + second;
    }
    println!("{}", acc);
}