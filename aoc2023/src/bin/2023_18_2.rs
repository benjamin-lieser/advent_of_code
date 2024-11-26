use advent_of_code::*;

fn main() {
    let input = get_input_aoc(18);

    let mut pos = (0isize, 0isize);

    let mut area = 0isize;

    let mut outside_points = 0isize;

    for comand in input.lines() {
        let s: [_;3] = split(comand);

        let dir: Dir = match &s[2][7..8] {
            "0" => Dir::Right,
            "1" => Dir::Down,
            "2" => Dir::Left,
            "3" => Dir::Up,
            _ => panic!()
        };
        let l: isize = isize::from_str_radix(&s[2][2..7], 16).unwrap();

        outside_points += l;

        let next = pos + l * dir;

        area += (pos.1 + next.1) * (pos.0 - next.0);

        pos = next;
    }

    let i2 = area - outside_points + 2;

    assert!(i2 % 2 == 0);

    println!("{}", outside_points + i2 / 2);

}
