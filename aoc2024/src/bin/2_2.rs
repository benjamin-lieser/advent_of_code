use advent_of_code::*;

fn safe(levels: &Vec<i64>) -> bool {
    if levels[0] < levels[1] {
        for i in 1..levels.len() {
            if levels[i] - levels[i - 1] < 1 || levels[i] - levels[i - 1] > 3 {
                return false;
            }
        }
    } else if levels[0] > levels[1] {
        for i in 1..levels.len() {
            if levels[i - 1] - levels[i] < 1 || levels[i - 1] - levels[i] > 3 {
                return false;
            }
        }
    } else {
        return false;
    }
    true
}

fn main() {
    let input = get_input(2024, 2);

    let mut acc = 0;

    'outer: for line in input.lines() {
        let levels = line.scast::<i64>("");
        if safe(&levels) {
            acc += 1;
            continue;
        }
        for i in 0..levels.len() {
            let mut copy = levels.clone();
            copy.remove(i);
            if safe(&copy) {
                acc += 1;
                continue 'outer;
            }
        }
    }
    print!("{}", acc);
}
