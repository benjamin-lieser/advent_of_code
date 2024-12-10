use advent_of_code::*;

fn count(grid: &[Vec<int>], pos: Pos) -> HashSet<Pos> {
    let current = index(grid, pos).unwrap();
    if current == 9 {
        let mut set = HashSet::new();
        set.insert(pos);
        return set;
    }
    let mut sum = HashSet::new();
    for dir in DIRS {
        let new_pos = pos + dir;
        if let Some(val) = index(grid, new_pos) {
            if val == current + 1 {
                sum.extend(count(grid, new_pos));
            }
        }
    }
    sum
}

fn main() {
    let input = get_input(2024, 10);

    let mut acc = 0;

    let grid = read_grid_int(&input);

    for (y, row) in grid.iter().enumerate() {
        for (x, &val) in row.iter().enumerate() {
            if val == 0 {
                acc += count(&grid, (x as int, y as int)).len();
            }
        }
    }


    println!("{}", acc);
}