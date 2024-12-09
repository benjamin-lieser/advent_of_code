use advent_of_code::*;

fn main() {
    let input = get_input(2024, 8);

    let mut acc = 0;

    let grid = read_grid(&input);

    let h = grid.len() as int;
    let w = grid[0].len() as int;

    let max = h.max(w) + 2;
    
    let mut locations = HashSet::<Pos>::new();

    for r in 0..h {
        for c in 0..w {
            if index(&grid, (c, r)) != Some('.') {
                for r2 in 0..h {
                    for c2 in 0..w {
                        if index(&grid, (c2, r2)) == index(&grid, (c, r)) && (c2, r2) != (c, r) {
                            let c_diff = c2 - c;
                            let r_diff = r2 - r;
                            
                            for i in -max..max {
                                locations.insert((c + i * c_diff, r + i * r_diff));
                            }


                        }
                    }
                }
            }
        }
    }

    for loc in locations {
        if index(&grid, loc).is_some() {
            acc += 1;
        }
    }

    println!("{}", acc);
}
