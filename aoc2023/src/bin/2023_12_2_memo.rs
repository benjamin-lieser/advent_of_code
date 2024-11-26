use advent_of_code::*;

/// How many possibilities
fn one_number(pattern: &[u8], number: usize) -> Vec<usize> {
    let mut positions = vec![];

    if number > pattern.len() {
        return positions;
    }

    for idx in 0..pattern.len() - number {
        if pattern[0..idx].iter().any(|&c| c == b'#') {
            continue;
        }

        if pattern[idx..idx+number].iter().any(|&c| c == b'.') {
            continue;
        }

        if pattern[idx+number] == b'#' {
            continue;
        }

        positions.push(idx);
    }
    positions
}

#[memoize::memoize]
fn dp(pattern_suffix: usize, number_suffix: usize, pattern: Vec<u8>, numbers: Vec<u8>) -> u64 {
    if pattern_suffix >= pattern.len() {
        if number_suffix >= numbers.len() {
            return 1;
        } else {
            return 0;
        }
    }

    if number_suffix >= numbers.len() {
        if pattern[pattern_suffix..].iter().all(|&c| c != b'#') {
            return 1;
        } else {
            return 0;
        }
    } else {
        
        let first_number = numbers[number_suffix] as usize;
        let mut possi = 0u64;
        for first_match in one_number(&pattern[pattern_suffix..], first_number) {
            possi += dp(pattern_suffix + first_match + first_number + 1, number_suffix + 1, pattern.clone(), numbers.clone());

        }
        possi
    }
}

fn main() {
    let input = get_input_aoc(12);

    //let input = ".??..??...?##. 1,1,3\n?#?#?#?#?#?#?#? 1,3,1,6\n????.#...#... 4,1,1\n????.######..#####. 1,6,5\n?###???????? 3,2,1";

    let mut acc = 0u64;

    for line in input.lines() {
        let (pattern, numbers) = line.split_once(' ').unwrap();
        let numbers: Vec<u8> = numbers.split(',').map(|n| n.parse().unwrap()).collect();

        let numbers: Vec<u8> = std::iter::repeat(numbers.iter()).take(5).flatten().copied().collect();

        let mut new_pattern = pattern.to_owned();

        for _ in 0..4 {
            new_pattern.push('?');
            new_pattern.push_str(pattern);
        }

        new_pattern.push('.');

        acc += dp(0,0,new_pattern.as_bytes().to_owned(), numbers);

    }

    println!("{}", acc);
}
