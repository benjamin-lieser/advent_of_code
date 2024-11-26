use std::vec;

use advent_of_code::*;

fn is_symmetry<S: AsRef<[u8]>>(pattern: &[S], axis: isize) -> bool {
    for (idx, line) in pattern.iter().enumerate() {
        let idx = idx as isize;
        let pair = (2 * axis - 1) - idx;
        if pair < 0 || pair >= pattern.len() as isize {
            continue;
        }
        if line.as_ref() != pattern[pair as usize].as_ref() {
            return false;
        }
    }
    true
}

fn find_symmetry<S: AsRef<[u8]>>(pattern: &[S]) -> Vec<usize> {
    let mut result = vec![];
    for i in 1..pattern.len() {
        if is_symmetry(pattern, i as isize) {
            result.push(i)
        }
    }
    result
}

fn mutate(array: &[&[u8]], i: usize, j: usize) -> Vec<Vec<u8>> {
    let col_length = array[0].len();
    let row_length = array.len();
    let mut result = vec![vec![0u8; col_length]; row_length];
    for r in 0..row_length {
        for c in 0..col_length {
            if r == i && c == j {
                result[r][c] = if array[r][c] == b'#' { b'.' } else { b'#' };
            } else {
                result[r][c] = array[r][c];
            }
        }
    }
    result
}

fn main() {
    let input = get_input_aoc(13);

    //let input = std::fs::read_to_string("data/2023_13").unwrap();

    let mut acc = 0usize;

    let lines: Vec<&[u8]> = input.lines().map(|l| l.as_bytes()).collect();

    'outer: for pattern in lines.split(|l| l.is_empty()) {
        let old_row = find_symmetry(pattern);
        let old_col = find_symmetry(&transpose(pattern));

        let old = (old_row, old_col);

        println!("old {:?}", old);
        for i in 0..pattern.len() {
            for j in 0..pattern[0].len() {
                let mutated = mutate(pattern, i, j);
                let mut row = find_symmetry(&mutated);
                let mut col = find_symmetry(&transpose(&mutated));


                if (row.clone(), col.clone()) != (vec![], vec![]) && (row.clone(), col.clone()) != old {
                    println!("old {:?} new {:?}", old, (row.clone(), col.clone()));

                    row.retain(|x| !old.0.contains(x));
                    col.retain(|x| !old.1.contains(x));

                    assert!(row.len() <= 1);
                    assert!(col.len() <= 1);

                    acc += 100 * row.first().unwrap_or(&0) + col.first().unwrap_or(&0);
                    continue 'outer;
                }
            }
        }
        panic!();
    }

    println!("{}", acc);
}
