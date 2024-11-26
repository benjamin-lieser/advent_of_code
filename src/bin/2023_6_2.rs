use advent_of_code::*;

fn main() {
    let input = get_input(2023, 6);

    let (time, distance) = input.so("\n");

    let time: u64 = time
        .chars()
        .filter(|x| x.is_digit(10))
        .collect::<String>()
        .parse()
        .unwrap();
    let distance: u64 = distance
        .chars()
        .filter(|x| x.is_digit(10))
        .collect::<String>()
        .parse()
        .unwrap();

    let acc = (0..time)
        .map(|press| press * (time - press))
        .filter(|x| x > &distance)
        .count() as u64;

    println!("{}", acc);
}
