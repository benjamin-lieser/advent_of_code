use advent_of_code::*;
use petgraph::visit::EdgeRef;

fn main() {
    let input = get_input(2024, 16);

    let mut grid = Grid::from_str(&input);

    let start = grid.find('S').unwrap();
    grid.set(start, '.');
    let end = grid.find('E').unwrap();
    grid.set(end, '.');

    let mut graph = DiGraphMap::<(Pos, Dir),int>::new();

    for pos in grid.positions() {
        for dir in DIRS {
            for turn in dir.turn_lr(){
                graph.add_edge((pos, dir), (pos, turn), 1000);
            }
            if grid.get(pos + dir) == Some('.') {
                graph.add_edge((pos, dir), (pos + dir, dir), 1);
            }
        }
    }

    let start = (start, Dir::Right);

    let dists = petgraph::algo::dijkstra(&graph, start, None, |e| *e.weight());

    let end_dist = DIRS.iter().map(|&dir| dists.get(&(end, dir)).unwrap()).min().unwrap();

    println!("{}", end_dist);
    
}
