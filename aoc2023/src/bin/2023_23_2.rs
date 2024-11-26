use advent_of_code::*;
use petgraph::visit::EdgeRef;

fn go(
    prev: Pos,
    grid: &[&[u8]],
    pos: Pos,
    dist: isize
) -> (Pos, Vec<Pos>, isize) {
    if pos.1 == grid.len() as isize - 1 {
        return (END, vec![], dist + 1);
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
        nexts.push(next);
    }

    if nexts.len() == 0 {
        return (pos, vec![], dist + 1);
    } else if nexts.len() == 1 {
        return go(pos, grid, nexts[0], dist + 1);
    } else {
        return (pos, nexts, dist + 1);
    }
}

fn get_edges(grid: &[&[u8]], start: Pos) -> Vec<(Pos, Pos, isize)> {
    let mut stack = vec![((-1,-1), vec![start])];

    let mut edges = vec![];

    let mut finished = vec![];

    while !stack.is_empty() {
        let (current, nexts) = stack.pop().unwrap();
        if finished.contains(&current) {
            continue;
        }
        
        for next in nexts.iter() {
            let (dest, nextnext, dist) = go(current, grid, *next, 0);

            edges.push((current, dest, dist));

            stack.push((dest, nextnext));
        }

        finished.push(current);
    }
    edges
}

fn main() {
    let input = get_input_aoc(23);

    //let input = std::fs::read_to_string("data/2023_23").unwrap();

    let grid: Vec<&[u8]> = input.lines().map(|l| l.as_bytes()).collect();

    let edges = get_edges(&grid, (1,0));

    let mut g = petgraph::graphmap::GraphMap::<Pos, isize, petgraph::Directed>::new();

    for (a, b, w) in edges {
        g.add_edge(a, b, w);
        g.add_edge(b, a, w);
        
    }

    let paths = petgraph::algo::simple_paths::all_simple_paths::<Vec<_>, _>(&g, (-1,-1), END, 0, None);

    let max = paths.map(|p| {
        let mut length = 0;
        let mut first = p[0];
        for node in p[1..].iter() {
            length += g.edge_weight(first, *node).unwrap();
            first = *node;
        }
        length
    }).max().unwrap();

    //let dist = petgraph::algo::floyd_warshall(&g, |e| *e.weight()).unwrap();

    //println!("{}", dist.get(&((1, 0), END)).unwrap());

    println!("{:?}", petgraph::dot::Dot::new(&g));

    println!("{}", max - 1);
}
