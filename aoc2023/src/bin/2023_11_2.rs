use advent_of_code::*;

fn main() {
    let input = get_input(2023, 11);

    let grid: Vec<&str> = input.lines().collect();

    let empty_cols: Vec<usize> = (0..grid[0].len())
        .filter(|idx| grid.iter().all(|row| row.as_bytes()[*idx] == b'.'))
        .collect();

    let empty_rows: Vec<usize> = grid
        .iter()
        .enumerate()
        .filter_map(|(idx, row)| {
            if row.chars().all(|c| c == '.') {
                Some(idx)
            } else {
                None
            }
        })
        .collect();

    let stars: Vec<(usize, usize)> = grid
        .iter()
        .enumerate()
        .map(|(y, row)| {
            row.chars()
                .enumerate()
                .filter_map(move |(x, c)| if c == '#' { Some((x, y)) } else { None })
        })
        .flatten()
        .collect();

    let mut acc = 0u64;

    for (x1, y1) in stars.iter() {
        for (x2, y2) in stars.iter() {
            if x1 < x2 {
                acc += (*x1..*x2).map(|x| if empty_cols.contains(&x) { 1_000_000 } else { 1 }).sum::<u64>();
            }

            if y1 < y2 {
                acc += (*y1..*y2).map(|y| if empty_rows.contains(&y) { 1_000_000 } else { 1 }).sum::<u64>();
            }
        }
    }

    println!("{}", acc);
}
