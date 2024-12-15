use advent_of_code::*;

fn calc(grid: &[Vec<u8>], start : (isize, isize), steps: isize) -> usize {
    let rows = grid.len() as isize;
    let cols = grid[0].len() as isize;
    
    let mut dp = vec![vec![false;cols as usize]; rows as usize];
    let mut dp2 = dp.clone();

    *index_mut(&mut dp, start).unwrap() = true;

    for _step in 1..=steps {
        dp2.iter_mut()
            .for_each(|r| r.iter_mut().for_each(|x| *x = false));
        for row in 0..grid.len() as isize {
            for col in 0..grid[0].len() as isize {
                let pos = (row, col);
                if !index(&dp, pos).unwrap() {
                    continue;
                }
                for dir in DIRS {
                    let next = pos + dir;
                    if let Some(c) = index(&grid, next) {
                        if c != b'#' {
                            *index_mut(&mut dp2, next).unwrap() = true;
                        }
                    }
                }
            }
        }
        std::mem::swap(&mut dp, &mut dp2);
    }

    let count = dp
        .iter()
        .map(|row| row.iter().filter(|&&x| x).count())
        .sum::<usize>();

    count

}

fn cache(c_vec : &mut [usize] ,grid: &[Vec<u8>], start : isize, mut steps: isize) -> usize {
    if steps < 0 {
        return 0;
    }
    if steps > 300 {
        if steps % 2 == 0 {
            steps = 300;
        } else {
            steps = 299;
        }
    }
    let index = (start + 4) * 400 + steps;
    if c_vec[index as usize] != usize::MAX {
        return c_vec[index as usize];
    } else {
        let r = (start + 4) / 3  - 1;
        let c = (start + 4) % 3 - 1;
        let result = calc(grid, (r * 65 + 65, c * 65 + 65), steps);
        c_vec[index as usize] = result;
        return result;
    }
}

fn diff(a: (isize, isize), b: (isize, isize)) -> isize{
    (a.0 - b.0).abs() + (a.1 - b.1).abs()
}

fn main() {
    let input = get_input(2023, 21);

    let grid: Vec<Vec<u8>> = input.lines().map(|l| l.as_bytes().to_owned()).collect();

    let rows = grid.len() as isize;
    let cols = grid[0].len() as isize;
    
    let steps = 26501365isize;

    let mut start = (0, 0);

    for (r, row) in grid.iter().enumerate() {
        if let Some((c, _)) = row.iter().enumerate().find(|(_, c)| **c == b'S') {
            start = (r as isize, c as isize);
            break;
        }
    }

    let start = start;

    println!("rows {}, cols {}, start {:?}", rows, cols, start);

    let mut acc = 0usize;

    let max = steps / rows + 5;

    let mut c_map = vec![usize::MAX; 4000];

    for r in -max..max {
        for c in -max..max {

            /*if r*r + c*c + 100 < max_2 {
                if (r + c) 
            }

            if r*r + c*c - 100 > max_2 {
                continue;
            }*/

            let rs = -r.signum();
            let cs = -c.signum();

            let entry_type = rs * 3 + cs;

            let row = r * rows;
            let col = c * cols;

            let entry = (row + rs * 65 + 65, col + cs * 65 + 65);

            acc += cache(&mut c_map, &grid, entry_type, steps - diff(start, entry))

        }

        println!("{}", r);
    }

    println!("{}", acc);
    
    println!("{:?}", c_map.retain(|f| *f != usize::MAX));
}
