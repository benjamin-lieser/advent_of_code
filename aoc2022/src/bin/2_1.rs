use advent_of_code::*;

fn main() {
    let input = get_input(2022, 2);

    let mut acc = 0;

    for line in input.lines() {
        let (op, my) = line.so(" ");
        let from_my = match my {
            "X" => 1,
            "Y" => 2,
            "Z" => 3,
            _ => panic!("Invalid my"),
        };
        let from_game = match my {
            "X" => match op {
                "A" => 3,
                "B" => 0,
                "C" => 6,
                _ => panic!("Invalid op"),
            },
            "Y" => match op {
                "A" => 6,
                "B" => 3,
                "C" => 0,
                _ => panic!("Invalid op"),
            },
            "Z" => match op {
                "A" => 0,
                "B" => 6,
                "C" => 3,
                _ => panic!("Invalid op"),
            },
            _ => panic!("Invalid my"),
        };
        acc += from_game + from_my;
    }

    println!("{}", acc);
}
