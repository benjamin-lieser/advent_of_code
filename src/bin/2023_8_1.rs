use advent_of_code::*;

fn main() {
    let input = get_input(2023, 8);

    let (lr, remainder) = input.so("\n");

    let map: HashMap<&str, (&str, &str)> = remainder.trim().lines().map(|line|{
        let (a, b) = line.split_once('=').unwrap();
        let paren = b.trim();
        let (c, d) = paren[1..=paren.len()-2].split_once(',').unwrap();
        (a.trim(), (c.trim(), d.trim()))
    }).collect();

    let mut current = "AAA";
    let mut acc = 0;

    for d in lr.trim().chars().cycle() {
        let next = map.get(current).unwrap();
        if d == 'L' {
            current = next.0;
        } else {
            current = next.1;
        }
        acc += 1;
        if current == "ZZZ" {
            break;
        }
    }
    
    println!("{}", acc);
}
