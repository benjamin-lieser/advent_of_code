use advent_of_code::*;

fn main() {
    let input = get_input(2022, 10);

    let mut acc = 0;

    let mut cycle = 0;

    let mut x = 1;

    let mut check = |cycle : int, x : int| {
        if (cycle - 20) % 40 == 0 {
            acc += x * cycle;
        }
    };

    for line in input.lines() {
        if line.starts_with("noop") {
            cycle += 1;
            check(cycle, x);
        } else {
            let [_, num] = split_s(line, "");
            let diff = num.parse::<int>().unwrap();
            cycle += 1;
            check(cycle, x);
            cycle += 1;
            check(cycle, x);
            x += diff;
        }
    }


    println!("{}", acc);
}
