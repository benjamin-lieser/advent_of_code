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

    for (a, b) in pairs.into_iter().take(1000) {
        union_find.union(a, b);
    }

    let mut groups = std::collections::HashMap::new();
    for i in 0..pos.len() {
        let root = union_find.find(i);
        groups.entry(root).or_insert(vec![]).push(i);
    }

    let mut all_groups = groups.values().collect::<Vec<_>>();

    all_groups.sort_by_key(|g| -(g.len() as int));

    println!(
        "{}",
        all_groups[0].len() * all_groups[1].len() * all_groups[2].len()
    );
}
