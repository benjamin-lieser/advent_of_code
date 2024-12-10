use advent_of_code::*;

fn move_north(col: &mut [u8]) {
    let pos: Vec<usize> = col
        .iter()
        .enumerate()
        .filter_map(|(idx, &c)| if c == b'O' { Some(idx) } else { None })
        .collect();
    'outer: for &rubble in pos.iter() {
        for j in (0..rubble).rev() {
            if col[j] == b'#' || col[j] == b'O' {
                col[rubble] = b'.';
                col[j + 1] = b'O';
                continue 'outer;
            }
        }

        col[rubble] = b'.';
        col[0] = b'O';
    }
}

fn main() {
    let input = get_input(2023, 14);

    let mut acc = 0usize;

    let lines: Vec<&[u8]> = input.lines().map(|l| l.as_bytes()).collect();

    let mut cols = transpose(&lines);

    for col in cols.iter_mut() {
        move_north(col);
    }

    let north = transpose(&cols);

    for (idx, line) in north.iter().enumerate() {
        let count = line.iter().filter(|&&c| c == b'O').count();
        acc += count * (north.len() - idx);
    }

    println!("{}", acc);
}
