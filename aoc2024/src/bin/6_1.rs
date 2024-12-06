use advent_of_code::*;

fn main() {
    let input = get_input(2024, 6);
    
    let mut acc = 0;

    let mut grid = read_grid(&input);

    let h = grid.len() as isize;
    let w = grid[0].len() as isize;

    let mut pos : Pos = (0,0);
    let mut dir : Dir = Dir::Up;

    for i in 0..h {
        for j in 0..w {
            if grid[i as usize][j as usize] == '^' {
                pos = (j,i);
            }
        }
    }

    loop {
        *index_mut(&mut grid, pos).unwrap() = 'X';
        let next = pos + dir;
        match index(&grid, next) {
            Some('#') => {
                dir = dir.turn_r();
            },
            None => {
                break;
            },
            Some(_) => {
                pos = next;
            },
        }
    }

    for i in 0..h {
        for j in 0..w {
            if index(&grid, (i,j)) == Some('X') {
                acc += 1;
            }
        }
    }

    println!("{}", grid.iter().map(|x| x.iter().collect::<String>()).collect::<Vec<String>>().join("\n"));
    
    println!("{}", acc);
}