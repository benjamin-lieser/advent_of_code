use advent_of_code::*;

fn main() {
    let input = get_input(2022, 8);

    let mut acc = 0;

    let grid = Grid::from_str(&input).to_int();

    for pos in grid.positions() {
        let mut vis = false;
        for dir in DIRS {
            let height = grid.get(pos).unwrap();
            let mut dir_vis = true;
            let mut pos = pos + dir;
            while let Some(next) = grid.get(pos) {
                if next >= height {
                    dir_vis = false;
                    break;
                }
                pos = pos + dir;
            }
            if dir_vis {
                vis = true;
            } 
        }
        if vis {
            acc += 1;
        }
    }


    println!("{}", acc);
}
