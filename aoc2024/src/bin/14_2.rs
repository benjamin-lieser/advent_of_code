use advent_of_code::*;

const X: int = 101;
const Y: int = 103;

fn main() {
    let input = get_input(2024, 14);

    let input : Vec<_> = input.lines().map(get_all_int::<4>).collect();

    let mut entropies = vec![];

    for i in 0..=X * Y {
        let mut grid = Grid::full(X, Y, '.');

        for [x,y,dx, dy] in &input {
            let posx = (x + dx * i).rem_euclid(X);
            let posy = (y + dy * i).rem_euclid(Y);
            grid.set((posx, posy), '#');
        }
        
        entropies.push(string_entropy(&grid.as_str(), 4));
    }

    println!("{}", entropies.argmin());
}
