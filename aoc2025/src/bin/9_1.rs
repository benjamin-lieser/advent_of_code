use advent_of_code::*;
use itertools::iproduct;

fn dist(a: &[int; 2], b: &[int; 2]) -> int {
    ((a[0] - b[0]).abs() + 1) * ((a[1] - b[1]).abs() + 1)
}

fn main() {
    let input = get_input(2025, 9);

    let pos: Vec<[int; 2]> = input.lines().map(|l| split(l, ",")).collect();

    let pairs = iproduct!(0..pos.len(), 0..pos.len())
        .filter(|(a, b)| a < b)
        .map(|(a, b)| (dist(&pos[a], &pos[b]), a, b))
        .sorted()
        .collect::<Vec<_>>();

    println!("{}", pairs[pairs.len() - 1].0);

}
