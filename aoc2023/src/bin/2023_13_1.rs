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

fn find_symmetry<S: AsRef<[u8]>>(pattern: &[S]) -> usize {
    for i in 1..pattern.len() {
        if is_symmetry(pattern, i as isize) {
            return i;
        }
    }
    0
}

fn main() {
    let input = get_input_aoc(13);

    let mut acc = 0usize;

    let lines: Vec<&[u8]> = input.lines().map(|l| l.as_bytes()).collect();

    for pattern in lines.split(|l| l.is_empty()) {
        acc += 100 * find_symmetry(pattern) + find_symmetry(&transpose(pattern))
    }

    println!("{}", acc);
}
