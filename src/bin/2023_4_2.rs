use advent_of_code::*;

fn main() {
    let input = get_input(2023, 4);
    let mut points = vec![];
    for line in input.lines() {
        
        let (_, cards) = line.split_once(':').unwrap();
        let (win, have) = cards.split_once('|').unwrap();

        let win = win.trim().split_ascii_whitespace().collect::<HashSet<&str>>();

        let count = have.trim().split_ascii_whitespace().filter(|num| win.contains(num)).count();

        points.push(count);
    }

    let mut copies = vec![1;points.len()];

    for (idx, point) in points.iter().enumerate() {
        for j in idx+1..idx+1+point {
            copies[j] += copies[idx];
        }
    }

    println!("{}", copies.iter().sum::<u64>());
}