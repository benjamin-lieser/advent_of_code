use advent_of_code::*;

fn main() {
    let input = get_input(2023, 4);
    let mut acc: int = 0;
    for line in input.lines() {
        let (_, cards) = line.split_once(':').unwrap();
        let (win, have) = cards.split_once('|').unwrap();

        let win = win.trim().split_ascii_whitespace().collect::<HashSet<&str>>();

        let count = have.trim().split_ascii_whitespace().filter(|num| win.contains(num)).count();

        acc += if count > 0 {1 << (count -1)} else {0};

    }
    println!("{}", acc);
}