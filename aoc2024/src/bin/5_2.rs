use advent_of_code::*;

fn main() {
    let input = get_input(2024, 5);

    let mut acc = 0;

    let lines = input.lines().collect::<Vec<&str>>();

    let mut parts = lines.split(|x| *x == "");
    let rules = parts.next().unwrap();
    let prints = parts.next().unwrap();

    let mut comp = HashSet::<(int, int)>::new();

    for rule in rules {
        let com = rule.so("|");
        comp.insert((com.0.parse().unwrap(), com.1.parse().unwrap()));
    }

    'outer: for print in prints {
        let numbers : Vec<int> = print.scast(",");
        for i in 0..numbers.len() {
            for j in i+1..numbers.len() {
                if comp.contains(&(numbers[j], numbers[i])){
                    continue 'outer;
                }
            }
        }
        acc += numbers[numbers.len() / 2];
    }

    println!("{}", acc);
}