use advent_of_code::*;

fn main() {
    let input = get_input(2022, 6);

    for i in 0..input.len() {
        let mut substr : Vec<u8> = input[i..i+14].as_bytes().to_vec();
        substr.sort();
        substr.dedup();
        if substr.len() == 14 {
            println!("{}", i+14);
            break;
        }
    }
}
