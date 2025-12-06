use advent_of_code::*;

fn main() {
    let input = get_input(2025, 6);

    let grid = Grid::from_str(&input);

    let mut counter = 0;

    let mut numbers = vec![];
    let mut operation = "".to_string();

    for x in 0..grid.c() {
        let mut col = vec![];
        for y in 0..grid.r() {
            col.push(grid.get((x, y)).unwrap());
        }
        if col.last().unwrap() != &' ' {
            operation = col.last().unwrap().to_string();
            col.pop();
        }
        let number = col.into_iter().collect::<String>();

        if number.trim().is_empty() {
            if operation == "*" {
                counter += numbers.iter().product::<i64>();
            } else if operation == "+" {
                counter += numbers.iter().sum::<i64>();
            }
            numbers.clear();
        } else {
            dbg!(&number);
            let number = number.trim().parse::<i64>().unwrap();
            numbers.push(number);
        }
    }
    if operation == "*" {
        counter += numbers.iter().product::<i64>();
    } else if operation == "+" {
        counter += numbers.iter().sum::<i64>();
    }

    println!("{}", counter);
}
