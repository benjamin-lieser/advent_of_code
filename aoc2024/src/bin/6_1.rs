use advent_of_code::*;

fn main() {
    let input = get_input(2024, 6);

    let mut acc = 0;

    let mut grid = read_grid(&input);

    let h = grid.len() as isize;
    let w = grid[0].len() as isize;

    let mut start: Pos = (0, 0);
    
    for i in 0..h {
        for j in 0..w {
            if grid[i as usize][j as usize] == '^' {
                start = (j, i);
            }
        }
    }

    let is_loop = |p: Pos| -> bool {
        let mut grid = grid.clone();
        *index_mut(&mut grid, p).unwrap() = '#';
        let mut pos = start;
        let mut dir = Dir::Up;
        for _ in 0..h*w+10 {
            let next = pos + dir;
            match index(&grid, next) {
                Some('#') => {
                    dir = dir.turn_r();
                }
                None => {
                    return false;
                }
                Some(_) => {
                    pos = next;
                }
            }
        }

        return true;
    };

    for i in 0..h {
        for j in 0..w {
            if (i,j) == start {
                continue;
            }
            if is_loop((j, i)) {
                acc += 1;
            }
        }
    }

    println!("{}", acc);
}
