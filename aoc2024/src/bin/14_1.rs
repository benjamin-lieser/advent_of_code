use advent_of_code::*;

const X: int = 101;
const Y: int = 103;

fn quad(x: int, y: int) -> int {
    if x > X/2 && y > Y/2 {
        0
    } else if x < X/2 && y > Y/2 {
        1
    } else if x < X/2 && y < Y/2 {
        2
    } else if x > X/2 && y < Y/2 {
        3
    } else {
        4
    }
}

fn main() {
    let input = get_input(2024, 14);

    let mut counter = [0, 0, 0, 0, 0];

    for line in input.lines() {
        let [x, y, dx, dy] = get_all_int(line);
        let posx = (x + dx * 100).rem_euclid(X);
        let posy = (y + dy * 100).rem_euclid(Y);
        dbg!(posx, posy);
        counter[quad(posx, posy) as usize] += 1;
    }

    dbg!(&counter);
    println!("{:?}", counter[0] * counter[1] * counter[2] * counter[3]);
}
