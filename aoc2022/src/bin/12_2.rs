use advent_of_code::*;

fn main() {
    let input = get_input(2022, 12);

    let grid = Grid::from_str(&input);

    let end = grid.find('E').unwrap();

    let grid = grid.map(|c| match c {
        'S' => 0,
        'E' => 26,
        x => x as int - 'a' as int,
    });

    let mut graph = DiGraphMap::new();

    for pos in grid.positions() {
        let cost = grid.get(pos).unwrap();
        for dir in DIRS {
            let next = pos + dir;
            if let Some(next_cost) = grid.get(next) {
                if next_cost <= cost + 1 {
                    graph.add_edge(pos, next, 1);
                }
            }
        }
    }

    let mut min = int::MAX;

    for start in grid.find_all(0) {
        let result = petgraph::algo::dijkstra(&graph, start, Some(end), |_| 1);
        if let Some(dist) = result.get(&end) {
            min = min.min(*dist);
        }
    }

    println!("{}", min);
}
