use advent_of_code::*;

fn main() {
    let input = get_input(2025, 1);

    let mut number = 50i32;

    let mut counter = 0;

    for line in input.lines() {
        let dir = line.chars().next().unwrap();
        let dist: i32 = line[1..].trim().parse().unwrap();

        let dir = match dir {
            'L' => -1,
            'R' => 1,
            _ => panic!("Unknown direction"),
        };

        for _ in 0..dist {
            number = number + dir;
            number %= 100;
            if number == 0 {
                counter += 1;
            }
        }
    }

    println!("{}", counter);

    
}