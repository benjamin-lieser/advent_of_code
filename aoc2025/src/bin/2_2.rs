use advent_of_code::*;

fn main() {
    let input = get_input(2025, 2);

    let mut counter = 0u64;

    let ranges = input.trim().split(',');

    for range in ranges {
        let bounds: Vec<u64> = range
            .split('-')
            .map(|s| s.parse().unwrap())
            .collect();
        let (start, end) = (bounds[0], bounds[1]);

        for n in start..=end {
            let digits: Vec<u32> = n
                .to_string()
                .chars()
                .map(|c| c.to_digit(10).unwrap())
                .collect();

            for i in 1..=(digits.len() / 2) {
                if digits.len() % i != 0 {
                    continue;
                }
                if digits[i..] == digits[..(digits.len() - i)] {
                    counter += n;
                    break;
                }
            }
        }
    }

    println!("{}", counter);
}
