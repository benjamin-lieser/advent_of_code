use advent_of_code::*;

fn main() {
    let input = get_input(2025, 7);

    let grid = Grid::from_str(&input);

    let mut counter = 0;

    let mut rays = HashSet::new();

    rays.insert(grid.find('S').unwrap().0);

    for i in 1..grid.r() {
        for r in rays.clone().iter() {
            if grid.get((*r, i)).unwrap() == '^' {
                counter += 1;
                rays.remove(r);
                rays.insert(*r - 1);

                rays.insert(*r + 1);
            }
        }
    }

    println!("{}", counter);
}
