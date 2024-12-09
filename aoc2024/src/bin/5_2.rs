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

    let sort = |num : &mut[int]| {
        for i in 0..num.len() {
            for j in i+1..num.len() {
                if comp.contains(&(num[j], num[i])) {
                    num.swap(i, j);
                }
            }
        }
    };

    for print in prints {
        let mut numbers : Vec<int> = print.scast(",");
        for i in 0..numbers.len() {
            for j in i+1..numbers.len() {
                if comp.contains(&(numbers[j], numbers[i])){
                    sort(&mut numbers);
                    acc += numbers[numbers.len() / 2];
                }
            }
        }
    }

    println!("{}", acc);
}