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
            if new_stones.contains_key(&n) {
                let count = new_stones.get(&n).unwrap();
                new_stones.insert(n, count + stone.1);
            } else {
                new_stones.insert(n, stone.1);
            }
        }
    }
    new_stones
}

fn main() {
    let input = get_input(2024, 11);

    let stones = input.scast("");

    let mut stones_map = HashMap::new();


    for stone in stones {
        if stones_map.contains_key(&stone) {
            let count = stones_map.get(&stone).unwrap();
            stones_map.insert(stone, count + 1);
        } else {
            stones_map.insert(stone, 1);
        }
    }
    for _ in 0..75 {
        stones_map = apply(stones_map);
    }

    println!("{}", stones_map.values().sum::<int>());
}