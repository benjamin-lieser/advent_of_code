use advent_of_code::*;

fn main() {
    let input = get_input(2023, 6);

    let (time, distance) = input.so("\n");

    let times : Vec<u64>= time.split_ascii_whitespace().skip(1).map(|x| x.parse().unwrap()).collect();
    let distances : Vec<u64>= distance.split_ascii_whitespace().skip(1).map(|x| x.parse().unwrap()).collect();

    let mut acc = 1u64;

    for (t, d) in times.iter().zip(distances) {
        acc *= (0..*t).map(|press| press * (t - press)).filter(|x| x > &d).count() as u64;
    }

    println!("{}", acc);
}
