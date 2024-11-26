use advent_of_code::*;

fn go(dp_array: &mut [Vec<[bool; 4]>], row: isize, col: isize, dir: usize, grid: &[&[u8]]) {
    if row >= grid.len() as isize || col >= grid[0].len() as isize || row < 0 || col < 0 {
        return;
    }

    if dp_array[row as usize][col as usize][dir] {
        return;
    } else {
        dp_array[row as usize][col as usize][dir] = true;

        match (grid[row as usize][col as usize], dir) {
            (b'.', 0) | (b'-', 0) => {
                go(dp_array, row, col + 1, dir, grid);
            }
            (b'.', 1) | (b'|', 1) => {
                go(dp_array, row + 1, col, dir, grid);
            }
            (b'.', 2) | (b'-', 2) => {
                go(dp_array, row, col - 1, dir, grid);
            }
            (b'.', 3) | (b'|', 3) => {
                go(dp_array, row - 1, col, dir, grid);
            }
            (b'-', 1) | (b'-', 3) => {
                go(dp_array, row, col - 1, 2, grid);
                go(dp_array, row, col + 1, 0, grid);
            }
            (b'|', 0) | (b'|', 2) => {
                go(dp_array, row - 1, col, 3, grid);
                go(dp_array, row + 1, col, 1, grid);
            }
            (b'/', 0) | (b'\\', 2) => go(dp_array, row - 1, col, 3, grid),
            (b'/', 1) | (b'\\', 3) => go(dp_array, row, col - 1, 2, grid),
            (b'/', 2) | (b'\\', 0) => go(dp_array, row + 1, col, 1, grid),
            (b'/', 3) | (b'\\', 1) => go(dp_array, row, col + 1, 0, grid),

            _ => panic!(),
        }
    }
}

fn calc(grid: &[&[u8]], row: isize, col: isize, dir: usize) -> usize {
    let mut dp_array = vec![vec![[false; 4]; grid[0].len()]; grid.len()];

    go(&mut dp_array, row, col, dir, grid);

    dp_array
        .iter()
        .map(|inner| inner.iter().map(|dirs| dirs.iter().any(|x| *x)))
        .flatten()
        .filter(|x| *x)
        .count()
}

fn main() {
    let input = get_input_aoc(16);

    let grid: Vec<&[u8]> = input.lines().map(|l| l.as_bytes()).collect();

    let mut acc = 0usize;

    for row in 0..grid.len() {
        acc = acc.max(calc(&grid, row as isize, 0, 0));
        acc = acc.max(calc(&grid, row as isize, grid[0].len() as isize - 1, 2));
    }
    for col in 0..grid[0].len() {
        acc = acc.max(calc(&grid, 0, col as isize, 1));
        acc = acc.max(calc(&grid, grid.len() as isize - 1, col as isize, 3));
    }

    println!("{}", acc);
}
