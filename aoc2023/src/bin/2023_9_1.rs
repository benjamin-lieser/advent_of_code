use advent_of_code::*;

fn main() {
    let input = get_input(2023, 9);

    let mut acc = 0i64;

    for line in input.lines() {
        let mut seq : Vec<i64> = line.scast("");
        let mut last_numbers = vec![];

        while seq.iter().any(|&x| x != 0) {
            for i in 0..seq.len() - 1 {
                seq[i] = seq[i+1] - seq[i];
            }
            last_numbers.push(seq.pop().unwrap())
        }

        acc += last_numbers.iter().sum::<i64>();

    }

    println!("{}", acc);
}
