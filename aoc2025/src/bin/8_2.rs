use advent_of_code::*;
use itertools::iproduct;
use union_find::{QuickUnionUf, UnionBySize, UnionFind};

fn dist(a: &[int; 3], b: &[int; 3]) -> int {
    (a[0] - b[0]).pow(2) + (a[1] - b[1]).pow(2) + (a[2] - b[2]).pow(2)
}

fn main() {
    let input = get_input(2025, 8);

    let pos: Vec<[int; 3]> = input.lines().map(|l| split(l, ",")).collect();

    let pairs = iproduct!(0..pos.len(), 0..pos.len())
        .filter(|(a, b)| a < b)
        .map(|(a, b)| (dist(&pos[a], &pos[b]), a, b))
        .sorted()
        .map(|(_, a, b)| (a, b))
        .collect::<Vec<_>>();


    let mut union_find = QuickUnionUf::<UnionBySize>::new(pos.len());

    let mut counter = 0;

    for (a, b) in pairs.into_iter() {
        if union_find.union(a, b) {
            counter += 1;
            if counter == pos.len() - 1 {
                println!("{}", pos[a][0] * pos[b][0]);
                return;
            }
        }
    }
}
