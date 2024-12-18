use advent_of_code::*;
use petgraph::visit::EdgeRef;

const X: int = 71;
const Y: int = 71;

fn main() {
    let input = get_input(2024, 18);

    let bytes: Vec<_> = input.lines().map(get_all_int::<2>).collect();

    for i in 1..bytes.len() {
        let mut grid = Grid::full(X, Y, '.');

        let mut graph = UnGraphMap::<Pos, int>::new();

        for &[x,y] in bytes.iter().take(i) {
            grid.set((x, y), '#');
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

        let start = (0, 0);

        let dists = petgraph::algo::dijkstra(&graph, start, None, |e| *e.weight());

        if dists.get(&(X - 1, Y - 1)).is_none() {
            println!("{:?}", bytes[i-1]);
            break;
        }
    }
}
