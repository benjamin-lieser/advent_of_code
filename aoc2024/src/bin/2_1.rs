use advent_of_code::*;

fn main() {
    let input = get_input_inf(2024, 2);

    let mut acc = 0;

    'outer: for line in input.lines() {
        let levels = line.scast::<i64>("");
        if levels[0] < levels[1] {
            for i in 1..levels.len() {
                if levels[i] - levels[i - 1] < 1 || levels[i] - levels[i - 1] > 3 {
                    continue 'outer;
                }
            }
        } else if levels[0] > levels[1] {
            for i in 1..levels.len() {
                if levels[i - 1] - levels[i] < 1 || levels[i - 1] - levels[i] > 3 {
                    continue 'outer;
                }
            }
        }
        else {
            continue;

        }
        acc += 1;
    }
    print!("{}", acc);
}