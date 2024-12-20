use advent_of_code::*;

fn main() {
    let input = get_input(2024, 20);

    let mut grid = Grid::from_str(&input);

    let start = grid.find('S').unwrap();
    let end = grid.find('E').unwrap();
    grid.set(start, '.');
    grid.set(end, '.');

    let mut path = vec![start];
    let mut current = start;
    let mut last = start;

    while current != end {
        let mut next = (0,0);
        for dir in DIRS {
            if current + dir != last && grid.get(current + dir) == Some('.') {
                next = current + dir;
            }
        }
        path.push(next);
        last = current;
        current = next;
    }

    let mut acc = 0;

    for i in 0..path.len() {
        for j in i+1..path.len() {
            let a = path[i];
            let b = path[j];
            let dist = manhattan(a, b);
            if dist <= 2 && (j - i) as int - dist >= 100 {
                acc += 1;
            }
        }
    }

    println!("{}", acc);
    
}