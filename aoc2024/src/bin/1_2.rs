use advent_of_code::*;

fn main() {
    let input = get_input(2024, 1);

    let mut left = vec![];
    let mut right = vec![];

    for line in input.lines() {
        let mut n = line.split_whitespace();
        left.push(n.next().unwrap().parse::<i32>().unwrap());
        right.push(n.next().unwrap().parse::<i32>().unwrap());
    }

    let mut acc  = 0u64;

    for l in left.iter() {
        acc += *l as u64 * right.iter().filter(|r| *r == l).count() as u64;
    }

    println!("{}", acc);
}