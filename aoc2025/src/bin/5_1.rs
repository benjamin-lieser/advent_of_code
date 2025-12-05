use advent_of_code::*;

fn main() {
    let input = get_input(2025, 5);

    let [ranges_s, ids_s] = split_s(&input, "\n\n");

    let ranges = ranges_s
        .lines()
        .map(|line| {
            let [a, b] = split_s(line, "-");
            (a.parse::<i64>().unwrap(), b.parse::<i64>().unwrap())
        })
        .collect::<Vec<_>>();

    let ids = ids_s
        .lines()
        .map(|line| line.parse::<i64>().unwrap())
        .collect::<Vec<_>>();

    let mut counter = 0;

    for id in ids {
        for (a, b) in &ranges {
            if id >= *a && id <= *b {
                counter += 1;
                break;
            }
        }
    }
    
    println!("{}", counter);
}
