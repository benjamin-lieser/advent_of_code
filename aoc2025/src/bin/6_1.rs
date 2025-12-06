use advent_of_code::*;

fn main() {
    let input = get_input(2025, 6);

    let lines = input.lines().collect::<Vec<_>>();

    let numbers = lines[..lines.len() - 1]
        .iter()
        .map(|line| line.split_ascii_whitespace()
            .map(|num| num.parse::<i64>().unwrap())
            .collect::<Vec<_>>())
        .collect::<Vec<_>>();

    let operations = lines[lines.len() - 1]
        .split_ascii_whitespace()
        .collect::<Vec<_>>();

    let mut counter = 0;

    for i in 0..numbers[0].len() {
        if operations[i] == "*" {
            counter += numbers.iter().map(|row| row[i]).product::<i64>();
        } else if operations[i] == "+" {
            counter += numbers.iter().map(|row| row[i]).sum::<i64>();
        }
    }
    
    println!("{}", counter);
}
