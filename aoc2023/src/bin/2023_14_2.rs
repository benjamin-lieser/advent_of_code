use std::thread::current;

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

fn do_cycle(lines: Vec<Vec<u8>>) -> Vec<Vec<u8>> {
    //north
    let mut cols = transpose(&lines);
    cols.iter_mut().for_each(|f| move_north(f));

    //west
    let mut lines = transpose(&cols);
    lines.iter_mut().for_each(|f| move_north(f));

    //south
    let mut cols = transpose(&lines);
    cols.iter_mut().for_each(|c| {
        c.reverse();
        move_north(c);
        c.reverse()
    });

    //east
    let mut lines = transpose(&cols);
    lines.iter_mut().for_each(|f| {
        f.reverse();
        move_north(f);
        f.reverse();
    });

    lines
}

fn main() {
    let input = get_input_aoc(14);

    //let input = std::fs::read_to_string("data/2023_14").unwrap();

    let mut acc = 0usize;

    let lines: Vec<Vec<u8>> = input.lines().map(Vec::<u8>::from).collect();

    let (offset, cycle) = cycle_detection(lines.clone(), do_cycle);

    println!("{},{}", offset, cycle);

    let n = 1_000_000_000usize;

    let n = (n - offset) % cycle + offset;

    let end = apply_n_times(do_cycle, n)(lines);

    for line in end.iter() {
        println!("{}", String::from_utf8(line.clone()).unwrap())
    }

    for (idx, line) in end.iter().enumerate() {
        let count = line.iter().filter(|&&c| c == b'O').count();
        acc += count * (end.len() - idx);
    }

    println!("{}", acc);
}
