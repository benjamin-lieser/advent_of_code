use advent_of_code::*;

fn main() {
    let input = get_input(2025, 7);

    let grid = Grid::from_str(&input);

    let mut poss = Grid::full_like(&grid, 0u64);

    poss.set(grid.find('S').unwrap(), 1);

    for y in 1..grid.r() {
        for x in 0..grid.c() {
            if grid.get((x, y)).unwrap() == '^' {
                *poss.get_mut((x - 1, y)).unwrap() += poss.get((x, y - 1)).unwrap();
                *poss.get_mut((x + 1, y)).unwrap() += poss.get((x, y - 1)).unwrap();
            } else {
                *poss.get_mut((x, y)).unwrap() += poss.get((x, y - 1)).unwrap();
            }
        }
    }

    poss.print_debug();

    let mut counter = 0;

    for x in 0..grid.c() {
        counter += poss.get((x, grid.r() - 1)).unwrap();
    }

    println!("{}", counter);
}
