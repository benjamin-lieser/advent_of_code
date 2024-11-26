use advent_of_code::*;

fn main() {
    let input = get_input(2023, 8);

    let (lr, remainder) = input.split_once('\n').unwrap();

    let map: HashMap<&str, (&str, &str)> = remainder
        .trim()
        .lines()
        .map(|line| {
            let (a, b) = line.split_once('=').unwrap();
            let paren = b.trim();
            let (c, d) = paren[1..=paren.len() - 2].split_once(',').unwrap();
            (a.trim(), (c.trim(), d.trim()))
        })
        .collect();

    let currents: Vec<&str> = map
        .keys()
        .into_iter()
        .filter(|s| s.chars().nth(2) == Some('A'))
        .cloned()
        .collect();
    println!("command length {}", lr.trim().len());

    let mut cycles = vec![];

    for mut current in currents.iter() {
        for (idx, d) in lr.trim().chars().cycle().enumerate() {
            let next = map.get(current).unwrap();
            if d == 'L' {
                current = &next.0;
            } else {
                current = &next.1;
            }
            if current.as_bytes()[2] == 'Z' as u8 {
                println!("{}", idx + 1);
                cycles.push(idx + 1);
                break;
            }
        }
    }

    println!("{}", lcm(&cycles));
}
