use advent_of_code::*;

const X: int = 101;
const Y: int = 103;

fn main() {
    let input = get_input(2024, 14);

    let mut i = 7753;

    for _ in 0..=50 {
        let mut grid = Grid::full(X, Y, ' ');

        for line in input.lines() {
            let [x, y, dx, dy] = get_all_int(line);
            let posx = (x + dx * i).rem_euclid(X);
            let posy = (y + dy * i).rem_euclid(Y);
            grid.set((posx, posy), '#');
        }
        println!("Time: {}", i);
        grid.print();
        i += 101;
    }
}
