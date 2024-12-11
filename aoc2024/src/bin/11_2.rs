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

fn apply(stones : HashMap<int, int>) -> HashMap<int, int> {
    let mut new_stones = HashMap::new();
    for stone in stones {
        let new = rule(stone.0);
        for n in new {
            *new_stones.entry(n).or_insert(0) += stone.1;
        }
    }
    new_stones
}

fn main() {
    let input = get_input(2024, 11);

    let stones = input.scast("");

    let mut stones_map = HashMap::new();


    for stone in stones {
        *stones_map.entry(stone).or_insert(0) += 1;
    }
    for _ in 0..75 {
        stones_map = apply(stones_map);
    }

    println!("{}", stones_map.values().sum::<int>());
}