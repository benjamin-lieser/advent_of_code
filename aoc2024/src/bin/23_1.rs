use advent_of_code::*;

fn main() {
    let input = get_input(2024, 23);

    let mut graph = UnGraphMap::new();

    for line in input.lines() {
        let [a,b] = split_s(line, "-");
        
        graph.add_edge(a, b, ());
    }

    let mut acc = 0;

    for node1 in graph.nodes() {
        for node2 in graph.nodes() {
            for node3 in graph.nodes() {
                if graph.contains_edge(node1, node2) && graph.contains_edge(node2, node3) && graph.contains_edge(node3, node1) {
                    if node1.starts_with("t") || node2.starts_with("t") || node3.starts_with("t") {
                        acc += 1;
                    }
                }
            }
        }
    }

    println!("{}", acc / 6);
    
}