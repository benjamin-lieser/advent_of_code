use advent_of_code::*;

fn main() {
    let input = get_input(2023, 24);

    //let input = std::fs::read_to_string("data/2023_24").unwrap();

    let (_start, mut vel): (Vec<_>, Vec<_>) = input
        .lines()
        .map(|l| {
            let (pos, v) = l.split_once('@').unwrap();
            let x: [isize; 3] = split(pos, ",");
            let v: [isize; 3] = split(v, ",");
            (x, v)
        })
        .unzip();

    vel.sort();

    println!("{:?}", vel);

    vel.dedup();

    println!("{}", vel.len());

    // Just put 3 lines into sympy and be happy :D
}
