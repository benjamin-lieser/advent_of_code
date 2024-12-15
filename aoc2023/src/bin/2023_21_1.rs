use advent_of_code::*;

fn main() {
    let input = get_input(2023, 21);

    let grid: Vec<Vec<u8>> = input.lines().map(|l| l.as_bytes().to_owned()).collect();

    let mut dp: Vec<Vec<bool>> = grid
        .iter()
        .map(|line| line.iter().map(|x| *x == b'S').collect())
        .collect();

    let mut dp2 = dp.clone();

    for _step in 1..=64 {
        dp2.iter_mut().for_each(|r| r.iter_mut().for_each(|x| *x = false));
        for row in 0..grid.len() as isize {
            for col in 0..grid[0].len() as isize {
                let pos = (row, col);
                if !index(&dp, pos).unwrap() {
                    continue;
                }
                for dir in DIRS {
                    let next = pos + dir;
                    if let Some(c) = index(&grid, next) {
                        if c != b'#' {
                            *index_mut(&mut dp2, next).unwrap() = true;
                        }
                    }
                }
            }
        }
        std::mem::swap(&mut dp, &mut dp2);
    }

    let count = dp.iter().map(|row| row.iter().filter(|&&x| x).count()).sum::<usize>();

    println!("{}", count);
}