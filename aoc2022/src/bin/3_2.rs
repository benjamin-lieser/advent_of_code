use advent_of_code::*;

fn main() {
    let input = get_input(2022, 3);

    let mut acc = 0;

    for mut lines in &input.lines().chunks(3) {
        let first = lines.next().unwrap();
        let second = lines.next().unwrap();
        let third = lines.next().unwrap();

        for (idx,c) in ('a'..='z').enumerate() {
            if first.contains(c) && second.contains(c) && third.contains(c) {
                acc += idx + 1;
                break;
            }
        }

        for (idx, c) in ('A'..='Z').enumerate() {
            if first.contains(c) && second.contains(c) && third.contains(c) {
                acc += idx + 27;
                break;
            }
        }
    }

    println!("{}", acc);
}
