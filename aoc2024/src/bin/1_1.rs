use advent_of_code::*;

fn main() {
    let input = get_input(2024, 1);

    let mut a = vec![];
    let mut b = vec![];

    for line in input.lines() {
        let mut n = line.split_whitespace();
        a.push(n.next().unwrap().parse::<i32>().unwrap());
        b.push(n.next().unwrap().parse::<i32>().unwrap());
    }

    a.sort();
    b.sort();

    println!("{}", a.iter().zip(b.iter()).map(|(a, b)| (a -b).abs()).sum::<i32>());
}