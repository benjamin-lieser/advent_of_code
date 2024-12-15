use advent_of_code::*;

fn make_move(grid: &mut Grid<char>, m: Dir, pos: Pos) -> Pos {
    if grid.get(pos + m) == Some('.') {
        return pos + m;
    } else if grid.get(pos + m) == Some('#') {
        return pos;
    } else {
        if move_box(grid, pos + m, m) {
            return pos + m;
        } else {
            return pos;
        }
    }
}

fn move_box(grid: &mut Grid<char>, pos: Pos, m: Dir) -> bool {
    if grid.get(pos + m) == Some('.') {
        grid.set(pos + m, 'O');
        grid.set(pos, '.');
        return true;
    } else if grid.get(pos + m) == Some('#') {
        return false;
    } else {
        if move_box(grid, pos + m, m) {
            grid.set(pos + m, 'O');
            grid.set(pos, '.');
            return true;
        } else {
            return false;
        }
    }
}

fn main() {
    let input = get_input(2024, 15);

    let (grid, moves) = input.so("\n\n");

    let mut grid = Grid::from_str(grid);
    let moves = moves
        .chars()
        .filter(|c| !c.is_whitespace())
        .collect::<String>();

    let mut start = grid.find('@').unwrap();
    grid.set(start, '.');

    for m in moves.chars() {
        start = make_move(&mut grid, Dir::from_char2(m).unwrap(), start);
    }

    let mut acc = 0;

    for pos in grid.positions() {
        if grid.get(pos) == Some('O') {
            acc += pos.0 + 100 * pos.1;
        }
    }

    println!("{}", acc);
}
