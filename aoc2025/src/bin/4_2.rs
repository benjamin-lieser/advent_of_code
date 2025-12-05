use advent_of_code::*;

fn main() {
    let input = get_input(2025, 4);

    let mut counter = 0;

    let mut grid = Grid::from_str(&input);
    loop {
        let mut changed = false;
        for pos in grid.positions() {
            let mut adv = 0;
            for dir in DIRS_DIAG {
                if grid.get(pos + dir).unwrap_or(' ') == '@' {
                    adv += 1;
                }
            }
            if adv < 4 && grid.get(pos).unwrap() == '@' {
                counter += 1;
                *grid.get_mut(pos).unwrap() = '.';
                changed = true;
            }
        }
        if !changed {
            break;
        }
    }
    println!("{}", counter);
}
