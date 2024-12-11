use advent_of_code::*;

fn rule(number : int) -> Vec<int> {
    if number == 0 {
        return vec![1];
    }
    let str = number.to_string();
    if str.len() % 2 == 0 {
        return vec![str[0..str.len()/2].parse().unwrap(), str[str.len()/2..].parse().unwrap()];
    } else {
        return vec![number * 2024];
    }
}

fn apply(stones : Vec<int>) -> Vec<int> {
    let mut new_stones = vec![];
    for stone in stones {
        new_stones.extend(rule(stone));
    }
    new_stones
}

fn main() {
    let input = get_input(2024, 11);

    let mut stones = input.scast("");

    
    for _ in 0..25 {
        stones = apply(stones);
    }

    println!("{}", stones.len());
}