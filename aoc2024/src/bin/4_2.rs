use advent_of_code::*;

fn main() {
    let input = get_input(2024, 4);

    let mut acc = 0;

    let grid = read_grid(&input);

    let h = grid.len() as isize;
    let w = grid[0].len() as isize;

    let dir = |i:isize,j:isize,dir:DirDiag| {
        let mut res = vec![index(&grid, (i,j))];
        for s in 1..4 {
            let (a,b) :(isize, isize) = dir.step();
            res.push(index(&grid, (i+s *a,j+s *b)));
        }
        return res;
    };

    for i in 0..h {
        for j in 0..w {
            for d in DIRS_DIAG {
                let word = dir(i,j,d);
                if word[0] == Some('X') && word[1] == Some('M') && word[2] == Some('A') && word[3] == Some('S') {
                    acc += 1;
                }
            }
        }
    }
    
    println!("{}", acc);
}