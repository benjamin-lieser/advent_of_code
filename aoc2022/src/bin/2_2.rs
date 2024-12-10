use advent_of_code::*;

fn main() {
    let input = get_input(2022, 2);

    let mut acc = 0;

    for line in input.lines() {
        let (op, result) = line.so(" ");
        let from_game = match result {
            "X" => match op {
                "A" => 3,
                "B" => 1,
                "C" => 2,
                _ => panic!("Invalid op"),
            },
            "Y" => match op {
                "A" => 4,
                "B" => 5,
                "C" => 6,
                _ => panic!("Invalid op"),
            },
            "Z" => match op {
                "A" => 8,
                "B" => 9,
                "C" => 7,
                _ => panic!("Invalid op"),
            },
            _ => panic!("Invalid my"),
        };
        acc += from_game;
    }

    println!("{}", acc);
}
