use advent_of_code::*;

#[memoize::memoize]
fn cost(seq: String, level: u8) -> int {
    if level == 3 {
        return seq.len() as int;
    }

    let grid = if level == 0 {
        KEYPAD.borrow()
    } else {
        CONTROL.borrow()
    };

    let mut sum = 0;

    let mut pos = if level == 0 { (2, 3) } else { (2, 0) };

    for c in seq.chars() {
        let next = where_is(c, grid);
        let paths = paths(pos, next, grid);
        let mut best = int::MAX;
        for path in paths {
            let new_seq = path + "A";
            let new_cost = cost(new_seq, level + 1);
            best = best.min(new_cost);
        }
        pos = next;
        sum += best;
    }

    sum
}

fn where_is(x: char, grid: &Grid<char>) -> Pos {
    grid.find(x).unwrap()
}

fn paths(start: Pos, end: Pos, grid: &Grid<char>) -> Vec<String> {
    let path = get_path(start, end)
        .iter()
        .map(|x| x.to_char2())
        .collect::<String>();

    let mut paths = string_permutations(&path);

    let is_valid = |path: &str| -> bool {
        let mut pos = start;
        for dir in path.chars() {
            let next = pos + Dir::from_char2(dir).unwrap();
            if grid.get(next).unwrap() == '#' {
                return false;
            }
            pos = next;
        }
        true
    };

    paths.retain(|x| is_valid(x));
    paths
}

static KEYPAD: Global<Grid<char>> = Global::new();
static CONTROL: Global<Grid<char>> = Global::new();

fn main() {
    let input = get_input(2024, 21);

    KEYPAD.set(Grid::from_str("789\n456\n123\n#0A\n"));
    CONTROL.set(Grid::from_str("#^A\n<v>\n"));

    let mut acc = 0;

    for line in input.lines() {
        let counts = cost(line.to_string(), 0);

        dbg!(counts);

        acc += line[..3].parse::<int>().unwrap() * counts;
    }

    dbg!(acc);
}
