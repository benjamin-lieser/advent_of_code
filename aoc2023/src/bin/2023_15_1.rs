use advent_of_code::*;

fn hash(com: &[u8]) -> u8 {
    com.iter()
        .fold(0u8, |acc, new| acc.wrapping_add(*new).wrapping_mul(17))
}

fn main() {
    let input = get_input(2023, 15);

    let mut acc = 0u64;

    for comand in input.trim().split_terminator(',') {
        acc += hash(comand.as_bytes()) as u64;
    }

    println!("{}", acc);
}
