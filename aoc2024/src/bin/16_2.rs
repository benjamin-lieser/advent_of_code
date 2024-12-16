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
        if grid.get(pos) == Some('#') {
            continue;
        }
        for dir in DIRS {
            for turn in dir.turn_lr(){
                graph.add_edge((pos, dir), (pos, turn), 1000);
            }
            if grid.get(pos + dir) == Some('.') {
                graph.add_edge((pos, dir), (pos + dir, dir), 1);
            }
        }
    }

    let start_state = (start, Dir::Right);

    let dists = petgraph::algo::dijkstra(&graph, start_state, None, |e| *e.weight());

    let mut tiles = HashSet::<Pos>::new();

    let end_dist = DIRS.iter().map(|&dir| dists.get(&(end, dir)).unwrap()).min().unwrap();

    for end_dir in DIRS {
        let back_dist = petgraph::algo::dijkstra(&graph, (end, end_dir.opp()), None, |e| *e.weight());
        for pos in grid.positions() {
            if grid.get(pos) == Some('#') {
                continue;
            }
            for possible_dir in DIRS {
                if back_dist.get(&(pos, possible_dir)).unwrap() + dists.get(&(pos, possible_dir.opp())).unwrap() == *end_dist {
                    tiles.insert(pos);
                }
            }
        }
    }


    println!("{}", tiles.len());
    
}
