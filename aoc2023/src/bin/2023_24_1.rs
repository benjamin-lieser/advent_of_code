use advent_of_code::*;

const MIN: i128 = 200_000_000_000_000;
const MAX: i128 = 400_000_000_000_000;
//const MIN: i128 = 7;
//const MAX: i128 = 24;

fn solve(x: [isize; 2], y: [isize; 2], v_x: [isize; 2], v_y: [isize; 2]) -> (isize, [isize; 2]) {
    let det = v_x[0] * v_y[1] - v_x[1] * v_y[0];

    let b = [y[0] - x[0], y[1] - x[1]];

    let sol = [
        b[0] * v_y[1] - b[1] * v_y[0],
        -(-b[0] * v_x[1] + b[1] * v_x[0]),
    ];
    (det, sol)
}

fn check(
    input: (isize, [isize; 2]),
    x: [isize; 2],
    y: [isize; 2],
    v_x: [isize; 2],
    v_y: [isize; 2],
) -> bool {
    let (det, sol) = input;
    let det = det as i128;
    if det == 0 {
        return false;
    }
    if det > 0 {
        if sol[0] < 0 || sol[1] < 0 {
            return false;
        } else {
            for i in 0..2 {
                let a = MIN * det <= x[i] as i128 * det + sol[0] as i128 * v_x[i] as i128;
                let b = x[i] as i128 * det + sol[0] as i128 * v_x[i] as i128 <= MAX * det;
                if !(a && b) {
                    return false;
                }
            }
            return true;
        }
    } else {
        if sol[0] > 0 || sol[1] > 0 {
            return false;
        } else {
            for i in 0..2 {
                let a = MIN * det >= x[i] as i128 * det + sol[0] as i128 * v_x[i] as i128;
                let b = x[i] as i128 * det + sol[0] as i128 * v_x[i] as i128 >= MAX * det;
                if !(a && b) {
                    return false;
                }
            }
            return true;
        }
    }
}

fn main() {
    let input = get_input(2024, 24);

    //let input = std::fs::read_to_string("data/2023_24").unwrap();

    let rays: Vec<_> = input
        .lines()
        .map(|l| {
            let (pos, v) = l.split_once('@').unwrap();
            let x: [isize; 2] = split(pos, ",");
            let v: [isize; 2] = split(v, ",");
            (x, v)
        })
        .collect();

    let mut acc = 0usize;

    for i in 0..rays.len() {
        for j in i + 1..rays.len() {
            let (x, v_x) = rays[i];
            let (y, v_y) = rays[j];

            let sol = solve(x, y, v_x, v_y);
            if check(sol, x, y, v_x, v_y) {
                acc += 1;
            }
        }
    }

    println!("{}", acc);
}
