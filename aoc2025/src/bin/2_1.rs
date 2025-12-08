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

            if digits.len() % 2 != 0 {
                continue;
            }
            if digits[0..(digits.len() / 2)] == digits[(digits.len() / 2)..] {
                counter += n;
            }
        }
    }

    println!("{}", counter);
}
