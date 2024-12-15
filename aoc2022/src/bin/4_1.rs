use advent_of_code::*;

fn main() {
    let input = get_input(2022, 4);

    let mut acc = 0;

    for line in input.lines() {
        let [a1,a2,b1,b2] = get_all_pos_int(line);
        if a1 >= b1 && a2 <= b2 {
            acc += 1;
        } else if b1 >= a1 && b2 <= a2 {
            acc += 1;
        }
    }

    println!("{}", acc);
}
