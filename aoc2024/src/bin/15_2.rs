use advent_of_code::*;

fn make_move(grid: &mut Grid<char>, m: Dir, pos: Pos) -> Pos {
    if grid.get(pos + m) == Some('.') {
        return pos + m;
    } else if grid.get(pos + m) == Some('#') {
        return pos;
    } else {
        if m == Dir::Left || m == Dir::Right {
            if move_box_h(grid, pos + m, m) {
                return pos + m;
            } else {
                return pos;
            }
        } else {
            if move_box_v(grid, vec![pos], m) {
                return pos + m;
            } else {
                return pos;
            }
        }
    }
}

fn move_box_v(grid: &mut Grid<char>, pos: Vec<Pos>, m: Dir) -> bool {
    if pos.iter().all(|x| grid.get(*x + m) == Some('.')) {
        for x in pos {
            grid.swap(x, x + m);
        }
        return true;
    } else if pos.iter().any(|x| grid.get(*x + m) == Some('#')) {
        return false;
    } else {
        let mut new_pos = pos.iter().map(|x| *x + m).collect::<Vec<_>>();
        extend_pos(grid, &mut new_pos);

        if move_box_v(grid, new_pos, m) {
            for x in pos {
                grid.swap(x, x + m);
            }
            return true;
        } else {
            return false;
        }
    }
}

fn extend_pos(grid: &Grid<char>, pos : &mut Vec<Pos>) {
    for p in pos.clone() {
        if grid.get(p) == Some('[') {
            if !pos.contains(&(p + Dir::Right)) {
                pos.push(p + Dir::Right);
            }
        }
        if grid.get(p) == Some(']') {
            if !pos.contains(&(p + Dir::Left)) {
                pos.push(p + Dir::Left);
            }
        }
    }
    pos.retain(|p| grid.get(*p) == Some('[') || grid.get(*p) == Some(']'));
}

fn move_box_h(grid: &mut Grid<char>, pos: Pos, m: Dir) -> bool {
    if grid.get(pos + m) == Some('.') {
        grid.swap(pos, pos + m);
        return true;
    } else if grid.get(pos + m) == Some('#') {
        return false;
    } else {
        if move_box_h(grid, pos + m, m) {
            grid.swap(pos, pos + m);
            return true;
        } else {
            return false;
        }
    }
}

fn main() {
    let input = get_input(2024, 15);

    let (grid, moves) = input.so("\n\n");

    let grid = grid.replace("#", "##");
    let grid = grid.replace("O", "[]");
    let grid = grid.replace(".", "..");
    let grid = grid.replace("@", "@.");

    let mut grid = Grid::from_str(&grid);
    let moves = moves
        .chars()
        .filter(|c| !c.is_whitespace())
        .collect::<String>();

    let mut start = grid.find('@').unwrap();
    grid.set(start, '.');

    for m in moves.chars() {
        start = make_move(&mut grid, Dir::from_char2(m).unwrap(), start);
        //grid.set(start, '@');
        //grid.print();
        //grid.set(start, '.');
    }

    grid.print();

    let mut acc = 0;

    for pos in grid.positions() {
        if grid.get(pos) == Some('[') {
            acc += pos.0 + 100 * pos.1;
        }
    }

    println!("{}", acc);
}
