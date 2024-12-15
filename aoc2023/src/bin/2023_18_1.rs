use advent_of_code::*;

fn main() {
    let input = get_input(2023, 18);

    let mut pos = (0isize, 0isize);

    let mut area = 0isize;

    let mut outside_points = 0isize;

    for comand in input.lines() {
        let s: [_;3] = split_s(comand, "");

        let dir: Dir = s[0].parse().unwrap();
        let l: isize = s[1].parse().unwrap();

        outside_points += l;

        let next = pos + l * dir;

        area += (pos.1 + next.1) * (pos.0 - next.0);

        pos = next;
    }

    let i2 = area - outside_points + 2;

    assert!(i2 % 2 == 0);

    println!("{}", outside_points + i2 / 2);

}
