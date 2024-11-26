use advent_of_code::*;

fn main() {
    let input = get_input(2023, 9);

    let mut acc = 0i64;

    for line in input.lines() {
        let mut seq : Vec<i64> = line.scast("");
        let mut first_numbers = vec![];

        first_numbers.push(seq[0]);

        while seq.iter().any(|&x| x != 0) {
            for i in 0..seq.len() - 1 {
                seq[i] = seq[i+1] - seq[i];
            }
            first_numbers.push(seq[0]);
            seq.pop();
        }

        acc += first_numbers.iter().rev().fold(0, |acc, new| new - acc);

    }

    println!("{}", acc);
}
