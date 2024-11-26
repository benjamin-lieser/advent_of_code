use std::collections::HashSet;

use advent_of_code::*;

fn main() {
    let input = get_input_aoc(11);

    let grid: Vec<&str> = input.lines().collect();

    let row_expand: Vec<&str> = grid
        .iter()
        .map(|row| {
            if row.chars().all(|c| c == '.') {
                std::iter::repeat(*row).take(2)
            } else {
                std::iter::repeat(*row).take(1)
            }
        })
        .flatten()
        .collect();

    let empty_cols: Vec<usize> = (0..grid[0].len())
        .filter(|idx| grid.iter().all(|row| row.as_bytes()[*idx] == b'.'))
        .collect();

    let expanded: Vec<String> = row_expand
        .iter()
        .map(|row| {
            row.chars()
                .enumerate()
                .map(|(idx, c)| {
                    if empty_cols.contains(&idx) {
                        std::iter::repeat(c).take(2)
                    } else {
                        std::iter::repeat(c).take(1)
                    }
                })
                .flatten()
                .collect()
        })
        .collect();

    let stars: HashSet<(usize, usize)> = expanded
        .iter()
        .enumerate()
        .map(|(idx, row)| {
            row.chars()
                .enumerate()
                .filter_map(move |(idx2, c)| if c == '#' { Some((idx, idx2)) } else { None })
        })
        .flatten()
        .collect();

    let mut acc = 0u64;

    for (x1,y1) in stars.iter() {
        for (x2, y2) in stars.iter() {
            acc += (*x1 as isize - *x2 as isize).abs() as u64;
            acc += (*y1 as isize - *y2 as isize).abs() as u64;
        }
    }

    println!("{}", acc / 2);
}
