use advent_of_code::*;
use petgraph::visit::EdgeRef;

fn main() {
    let input = get_input(2024, 20);

    let mut grid = Grid::from_str(&input);

    let start = grid.find('S').unwrap();
    let end = grid.find('E').unwrap();
    grid.set(start, '.');
    grid.set(end, '.');

    let mut graph = UnGraphMap::<Pos,int>::new();

    for pos in grid.positions() {
        if grid.get(pos) == Some('#') {
            continue;
        }
        for dir in DIRS {
            if grid.get(pos + dir) == Some('.') {
                graph.add_edge(pos, pos + dir, 1);
            }
        }
    }
    let dists = petgraph::algo::dijkstra(&graph, start, None, |e| *e.weight());

    let orig = dists.get(&end).unwrap();

    let mut cheats : HashMap<int, Vec<(Pos, Pos)>> = HashMap::new();

    for pos in grid.positions() {
        if grid.get(pos) == Some('#') {
            continue;
        }
        for dir in DIRS {
            let pos2 = pos + 2 * dir;
            if grid.get(pos2) == Some('.') {
                let mut graph2 = graph.clone();
                graph2.add_edge(pos, pos2, 2);
                let dists2 = petgraph::algo::dijkstra(&graph2, start, Some(end), |e| *e.weight());
                let dist = dists2.get(&end).unwrap();
                cheats.entry(orig - dist).or_default().push((pos, pos2));
            }
        }
    }

    let mut acc = 0;

    for dist in cheats.keys() {
        if *dist >= 100 {
            acc += cheats[dist].len() / 2;
        }
    }

    println!("{}", acc);
    
}