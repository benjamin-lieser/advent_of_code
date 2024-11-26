use advent_of_code::*;
use petgraph::visit::EdgeRef;

fn go(
    prev: Pos,
    grid: &[&[u8]],
    pos: Pos,
    cross: &mut Vec<(Pos, usize, Pos)>,
    last_corss: (usize, Pos),
) {
    if pos.1 == grid.len() as isize - 1 {
        cross.push((END, last_corss.0, last_corss.1));
        return;
    }

    if pos == (13, 13) {
        println!();
    }

    let mut nexts = vec![];

    for dir in DIRS {
        let next = pos + dir;
        let field = index(grid, next);
        if field.is_none() || field.unwrap() == b'#' {
            continue;
        }
        if next == prev {
            continue;
        }

        if match dir {
            Dir::Up => field.unwrap() == b'v',
            Dir::Right => field.unwrap() == b'<',
            Dir::Down => field.unwrap() == b'^',
            Dir::Left => field.unwrap() == b'>',
        } {
            continue;
        }
        nexts.push(next);
    }

    if nexts.len() == 0 {
        return;
    } else if nexts.len() == 1 {
        go(pos, grid, nexts[0], cross, (last_corss.0 + 1, last_corss.1));
    } else {
        cross.push((pos, last_corss.0, last_corss.1));
        for n in nexts.iter() {
            go(pos, grid, *n, cross, (1, pos));
        }
    }
}

fn main() {
    let input = get_input(23);

    //let input = std::fs::read_to_string("data/2023_23").unwrap();

    let grid: Vec<&[u8]> = input.lines().map(|l| l.as_bytes()).collect();

    let rows = grid.len();
    let cols = grid[0].len();

    let mut visited = vec![vec![false; cols]; rows];

    let mut cross = vec![];

    go((-1, -1), &grid, (1, 0), &mut cross, (0, (1, 0)));

    println!("{:?}", cross);

    let mut g = petgraph::graphmap::GraphMap::<Pos, isize, petgraph::Directed>::new();

    for c in cross.iter() {
        if let Some(e) = g.edge_weight(c.2, c.0) {
            if *e > -(c.1 as isize) {
                g.add_edge(c.2, c.0, -(c.1 as isize));
            }
        } else {
            g.add_edge(c.2, c.0, -(c.1 as isize));
        }
    }

    let dist = petgraph::algo::floyd_warshall(&g, |e| *e.weight()).unwrap();

    println!("{}", dist.get(&((1, 0), END)).unwrap());

    println!("{:?}", petgraph::dot::Dot::new(&g));
}
