use advent_of_code::*;
use petgraph::visit::EdgeRef;

const X : int = 71;
const Y : int = 71;

fn main() {
    let input = get_input(2024, 18);

    let mut grid = Grid::full(X, Y, '.');

    let mut graph = UnGraphMap::<Pos,int>::new();

    for line in input.lines().take(1024) {
        let [x,y] = get_all_int(line);
        grid.set((x,y), '#');
    }

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

    let start = (0,0);

    let dists = petgraph::algo::dijkstra(&graph, start, None, |e| *e.weight());

    println!("{}", dists.get(&(X-1,Y-1)).unwrap());
    
}
