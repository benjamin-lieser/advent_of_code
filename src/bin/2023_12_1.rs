use advent_of_code::*;

fn check(sample: u32, mut numbers: &[u8], n: usize) -> bool {
    let mut current = 0;
    for idx in 0..n+1 {
        if sample & 1 << idx > 0 {
            current += 1;
        } else {
            if current > 0 {
                if numbers.len() > 0 && numbers[0] == current {
                    numbers = &numbers[1..];
                    current = 0;
                } else {
                    return false;
                }
            }
        }
    }
    return numbers.len() == 0;
}

fn main() {
    let input = get_input(2023,12);

    let mut acc = 0i64;

    for line in input.lines() {
        let (pattern, numbers) = line.split_once(' ').unwrap();
        let numbers: Vec<u8> = numbers.split(',').map(|n| n.parse().unwrap()).collect();

        let mask = pattern
            .chars()
            .enumerate()
            .map(|(idx, c)| if c == '?' { 1u32 << idx } else { 0u32 })
            .sum::<u32>();
        let known = pattern
            .chars()
            .enumerate()
            .map(|(idx, c)| if c == '#' { 1u32 << idx } else { 0u32 })
            .sum::<u32>();

        let n = pattern.len();

        for sample in 0..1u32 << n {
            if !(sample ^ known) | mask == u32::MAX {
                if check(sample, &numbers, n) {
                    acc += 1;
                }
            }
        }
    }

    println!("{}", acc);
}
