use advent_of_code::*;

fn is_clique(graph: &UnGraphMap<&str, ()>, clique: &[&str]) -> bool {
    for node1 in clique {
        for node2 in clique {
            if node1 != node2 && !graph.contains_edge(*node1, *node2) {
                return false;
            }
        }
    }

    true
}
fn main() {
    let input = get_input(2024, 23);

    let mut graph = UnGraphMap::new();

    for line in input.lines() {
        let [a, b] = split_s(line, "-");

        graph.add_edge(a, b, ());
    }

    let mut lan = vec![];

    for node in graph.nodes() {
        let nexts = graph.neighbors(node).collect::<Vec<_>>();
        for i in 0..nexts.len() {
            let mut select = nexts.clone();
            select.remove(i);
            if is_clique(&graph, &select) {
                lan.push(node);
                break;
            }
        }
    }

    lan.sort();

    for a in lan {
        print!("{},", a);
    }
}
