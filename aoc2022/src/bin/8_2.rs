use advent_of_code::*;

fn main() {
    let input = get_input(2022, 8);

    let mut max = 0;

    let grid = Grid::from_str(&input).to_int();

    for pos in grid.positions() {
        let scores = DIRS.map(|dir| {
            let height = grid.get(pos).unwrap();
            let mut count = 0;
            let mut pos = pos + dir;
            while let Some(next) = grid.get(pos) {
                count += 1;
                if next >= height {
                    break;
                }
                pos = pos + dir;
            }
            count
        });

        max = max.max(scores.iter().product());
    }


    println!("{}", max);
}
