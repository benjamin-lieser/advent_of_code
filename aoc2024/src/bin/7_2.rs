use advent_of_code::*;

fn can_be_solved(result : int, numbers : &[int], current : int) -> bool {
    if numbers.len() == 0 {
        return result == current;
    }
    if can_be_solved(result, &numbers[1..], current + numbers[0]) {
        return true;
    }
    if can_be_solved(result, &numbers[1..], current * numbers[0]) {
        return true;
    }
    let concat : int= format!("{}{}", current, numbers[0]).parse().unwrap();
    if can_be_solved(result, &numbers[1..], concat) {
        return true;
    }
    false
}

fn main() {
    let input = get_input(2024, 7);

    let mut acc = 0;

    for line in input.lines() {
        let (result, numbers) = line.so(":");
        let numbers : Vec<int> = numbers.scast("");
        if can_be_solved(result.parse().unwrap(), &numbers[1..], numbers[0]) {
            acc += result.parse::<int>().unwrap();
        }
    }

    println!("{}", acc);
}