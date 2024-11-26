use advent_of_code::*;

fn main() {
    let input = get_input(2015, 1);

    let mut acc = 0;

    for (idx,c) in input.chars().enumerate() {
        if c == '(' {
            acc += 1;
        } else {
            acc -= 1;
        }

        if acc < 0 {
            println!("{}", idx + 1);
            return;
        }
    }
}