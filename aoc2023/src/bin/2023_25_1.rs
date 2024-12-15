use advent_of_code::*;
use graphalgs::connect::find_bridges;
use petgraph::{algo::tarjan_scc, visit::EdgeRef};

fn main() {
    let input = get_input(2023, 25);

    //let input = std::fs::read_to_string("data/2023_25").unwrap();

    let mut g = petgraph::graphmap::GraphMap::<&str, (), petgraph::Undirected>::new();

    for line in input.lines() {
        let (start, ends) = line.split_once(':').unwrap();

        for end in ends.split_ascii_whitespace() {
            g.add_edge(start, end, ());
        }
    }

    let g = g.into_graph::<u32>();

    for e1 in g.edge_indices() {
        for e2 in g.edge_indices() {
            let mut copy = g.clone();
            copy.remove_edge(e1);
            copy.remove_edge(e2);

            let bridges = find_bridges(&copy);

            if bridges.len() > 0 {
                let bridge = copy
                    .edges_connecting(bridges[0].0, bridges[0].1)
                    .next()
                    .unwrap();
                copy.remove_edge(bridge.id());

                let components = tarjan_scc(&copy);

                assert!(components.len() == 2);
                println!("{}", components[0].len() * components[1].len())
            }
        }
    }

    //let smaller = condensation(g, false);

    //println!("{:?}", petgraph::dot::Dot::new(&g));

    println!("Nodes: {}, Edges: {}", g.node_count(), g.edge_count());
}
