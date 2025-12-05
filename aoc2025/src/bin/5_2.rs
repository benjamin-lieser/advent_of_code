use advent_of_code::*;

fn main() {
    let input = get_input(2025, 5);

    let [ranges_s, _ids_s] = split_s(&input, "\n\n");

    let mut ranges = ranges_s
        .lines()
        .map(|line| {
            let [a, b] = split_s(line, "-");
            (a.parse::<i64>().unwrap(), b.parse::<i64>().unwrap())
        })
        .collect::<Vec<_>>();

    ranges.sort();

    let mut counter = 0;

    let mut current_biggest_range = (0, 0);

    for (a,b) in ranges {
        if a > current_biggest_range.1 {
            current_biggest_range = (a, b);
            counter += b - a + 1;
        } else {
            if b > current_biggest_range.1 {
                counter += b - current_biggest_range.1;
                current_biggest_range.1 = b;
            }
        }
    }
    
    println!("{}", counter);
}
