use advent_of_code::*;

fn main() {
    let input = get_input(2022, 3);

    let mut acc = 0;

    for line in input.lines() {
        let first = &line[0..line.len()/2];
        let second = &line[line.len()/2..];

        for (idx,c) in ('a'..='z').enumerate() {
            if first.contains(c) && second.contains(c) {
                acc += idx + 1;
                break;
            }
        }

        for (idx, c) in ('A'..='Z').enumerate() {
            if first.contains(c) && second.contains(c) {
                acc += idx + 27;
                break;
            }
        }
    }

    println!("{}", acc);
}
