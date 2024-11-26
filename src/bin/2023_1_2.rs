use advent_of_code::*;

fn main() {
    let input = get_input(2023, 1);

    let numbers: HashMap<&str, u64> = HashMap::from([
        ("0", 0),
        ("1", 1),
        ("2", 2),
        ("3", 3),
        ("4", 4),
        ("5", 5),
        ("6", 6),
        ("7", 7),
        ("8", 8),
        ("9", 9),
        ("one", 1),
        ("two", 2),
        ("three", 3),
        ("four", 4),
        ("five", 5),
        ("six", 6),
        ("seven", 7),
        ("eight", 8),
        ("nine", 9),
    ]);

    let mut acc: u64 = 0;
    for line in input.lines() {

        let digits: BTreeMap<usize, u64> = numbers
            .iter()
            .map(|(&pattern, &number)| {
                line.match_indices(pattern)
                    .map(move |(idx, _)| (idx, number))
            })
            .flatten()
            .collect();

        let first = *digits.first_key_value().unwrap().1;
        let second = *digits.last_key_value().unwrap().1;
        acc += first * 10 + second;
    }
    println!("{}", acc);
}
