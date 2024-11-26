use advent_of_code::*;
use petgraph::{algo::dijkstra, graphmap::GraphMap, visit::EdgeRef, Directed};

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy)]
struct Node {
    pos: (isize, isize),
    entered_from: Dir,
    straight_since: usize,
}

fn add(g: &mut GraphMap<Node, usize, Directed>, src: Node, dst: Node, gird: &[Vec<usize>]) {
    let max = (gird.len(), gird[0].len());
    if dst.pos.0 < 0 || dst.pos.1 < 0 || dst.pos.0 >= max.0 as isize || dst.pos.1 >= max.1 as isize
    {
        return;
    }
    g.add_edge(src, dst, gird[dst.pos.0 as usize][dst.pos.1 as usize]);
}

fn main() {
    let input = get_input_aoc(17);

    //let input = std::fs::read_to_string("data/2023_17").unwrap();

    let grid: Vec<Vec<usize>> = input
        .lines()
        .map(|line| line.chars().map(|c| c as usize - '0' as usize).collect())
        .collect();

    let mut g = petgraph::graphmap::GraphMap::<Node, usize, Directed>::new();

    for row in 0..grid.len() as isize {
        for col in 0..grid[0].len() as isize {
            for dir in DIRS.into_iter() {
                for s_left in 0..=10 as usize {
                    let node = Node {
                        pos: (row, col),
                        entered_from: dir,
                        straight_since: s_left,
                    };
                    g.add_node(node);
                }
            }
        }
    }

    for row in 0..grid.len() as isize {
        for col in 0..grid[0].len() as isize {
            for dir in DIRS.into_iter() {
                for s_since in 0..=10 as usize {
                    let node = Node {
                        pos: (row, col),
                        entered_from: dir,
                        straight_since: s_since,
                    };

                    //straight
                    if s_since < 10 {
                        let next = Node {
                            pos: (row, col) + dir,
                            entered_from: dir,
                            straight_since: s_since + 1,
                        };
                        add(&mut g, node, next, &grid);
                    }
                    //left right
                    if s_since >= 4 {
                        for turn in dir.turn_lr().into_iter() {
                            let (rd, cd) = turn.d::<isize>();
                            let next = Node {
                                pos: (row + rd, col + cd),
                                entered_from: turn,
                                straight_since: 1,
                            };
                            add(&mut g, node, next, &grid);
                        }
                    }
                }
            }
        }
    }

    let start = Node {
        pos: (isize::MAX, isize::MAX),
        entered_from: Dir::Right,
        straight_since: usize::MAX,
    };
    let end = Node {
        pos: (isize::MAX, isize::MAX),
        entered_from: Dir::Left,
        straight_since: usize::MAX,
    };

    g.add_node(start);
    g.add_node(end);

    g.add_edge(
        start,
        Node {
            pos: (0, 0),
            entered_from: Dir::Right,
            straight_since: 1,
        },
        0,
    );
    g.add_edge(
        start,
        Node {
            pos: (0, 0),
            entered_from: Dir::Down,
            straight_since: 1,
        },
        0,
    );

    for s_since in 4..=10 {
        for dir in DIRS.into_iter() {
            let node = Node {
                pos: (grid.len() as isize - 1, grid[0].len() as isize - 1),
                entered_from: dir,
                straight_since: s_since,
            };
            g.add_edge(node, end, 0);
        }
    }

    let costs = dijkstra(&g, start, Some(end), |e| *e.weight());

    //println!("{:?}", costs);

    println!("{:?}", costs.get(&end))
}
