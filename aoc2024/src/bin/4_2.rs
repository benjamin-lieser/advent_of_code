use advent_of_code::*;

fn main() {
    let input = get_input(2024, 4);

    let mut acc = 0;

    let grid = read_grid(&input);

    let h = grid.len() as isize;
    let w = grid[0].len() as isize;

    let corners = |i: isize, j: isize| {
        return vec![
            index(&grid, (i - 1, j - 1)),
            index(&grid, (i - 1, j + 1)),
            index(&grid, (i + 1, j - 1)),
            index(&grid, (i + 1, j + 1)),
        ];
    };

    for i in 0..h {
        for j in 0..w {
            if index(&grid, (i, j)) != Some('A') {
                continue;
            }
            let word = corners(i, j);
            if word[0] == Some('M')
                && word[1] == Some('M')
                && word[2] == Some('S')
                && word[3] == Some('S')
            {
                acc += 1;
                continue;
            }
            if word[0] == Some('M')
                && word[1] == Some('S')
                && word[2] == Some('M')
                && word[3] == Some('S')
            {
                acc += 1;
                continue;
            }
            if word[0] == Some('S')
                && word[1] == Some('M')
                && word[2] == Some('S')
                && word[3] == Some('M')
            {
                acc += 1;
                continue;
            }
            if word[0] == Some('S')
                && word[1] == Some('S')
                && word[2] == Some('M')
                && word[3] == Some('M')
            {
                acc += 1;
                continue;
            }
        }
    }

    println!("{}", acc);
}
