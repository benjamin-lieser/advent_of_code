use advent_of_code::*;

fn main() {
    let input = get_input(2025, 4);

    let mut counter = 0;

    let grid = Grid::from_str(&input);

    for pos in grid.positions() {
        let mut adv = 0;
        for dir in DIRS_DIAG {
            if grid.get(pos + dir).unwrap_or(' ') == '@' {
                adv += 1;
            }
        }
        if adv < 4 && grid.get(pos).unwrap() == '@' {
            counter += 1;
        }
    }
    println!("{}", counter);
}
