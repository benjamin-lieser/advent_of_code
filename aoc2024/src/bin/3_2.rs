use advent_of_code::*;
use regex::Regex;

fn main() {
    let input = get_input(2024, 3);

    let mut acc = 0;

    let mul_regex = Regex::new(r"mul\((\d+),(\d+)\)").unwrap();
    let do_regex = Regex::new(r"do\(\)").unwrap();
    let dont_regex = Regex::new(r"don't\(\)").unwrap();

    let mut muls : Vec<(usize, i64)> = vec![];

    for cap in mul_regex.captures_iter(&input) {
        let index = cap.get(0).unwrap().start();
        let a = cap[1].parse::<i64>().unwrap();
        let b = cap[2].parse::<i64>().unwrap();
        muls.push((index, a * b));
    }

    for m in do_regex.find_iter(&input) {
        let index = m.start();
        muls.push((index, -1));
    }

    for m in dont_regex.find_iter(&input) {
        let index = m.start();
        muls.push((index, -2));
    }

    muls.sort();

    let mut enabled = true;

    for (_, mul) in muls {
        if mul == -1 {
            enabled = true;
        } else if mul == -2 {
            enabled = false;
        } else {
            if enabled {
                acc += mul;
            }
        }
    }

    println!("{}", acc);
}