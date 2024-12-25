use advent_of_code::*;

fn main() {
    let input = get_input(2024, 25);

    let mut acc = 0;

    let mut keys = vec![];
    let mut locks = vec![];

    for block in split_empty_line(&input) {
        let mut grid = Grid::from_str(block);
        let is_key = grid.get((0,0)).unwrap() == '.';
        if is_key {
            grid = grid.flip_y();
        }
        let hight_profile = (0..5).map(|i| grid.count_col(i, '#') - 1).collect::<Vec<_>>();
        if is_key {
            keys.push(hight_profile);
        } else {
            locks.push(hight_profile);
        }
    }
    
    for key in keys {
        for lock in locks.iter() {
            if key.iter().zip(lock.iter()).all(|(a,b)| a + b < 6) {
                acc += 1;
            }
        }
    }

    println!("{}", acc);

}