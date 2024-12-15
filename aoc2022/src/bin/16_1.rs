use advent_of_code::*;

#[memoize::memoize]
fn max_release(
    graph: &'static UnGraphMap<usize, ()>,
    flows: &'static HashMap<usize, int>,
    pos: usize,
    time_left: usize,
    open: HashMap<usize, bool>,
) -> int {
    if time_left == 0 {
        return 0;
    }
    let mut max = 0;
    if open.get(&pos) == Some(&false) {
        let release: int = open
            .iter()
            .filter(|(_, v)| **v == true)
            .map(|(k, _)| flows.get(k).unwrap())
            .sum();
        let mut new_open = open.clone();
        *new_open.get_mut(&pos).unwrap() = true;
        let future = max_release(graph, flows, pos, time_left - 1, new_open);
        return release + future;
    }

    todo!()
}

fn main() {
    let input = get_input(2022, 16);

    let mut indexer = Indexer::new();

    let mut edges = vec![];

    let mut flows = HashMap::<usize, int>::new();

    for line in input.lines() {
        let [flow] = get_all_int(&line);
        let nodes = get_all_matches(&line, "[A-Z][A-Z]")
            .iter()
            .map(|&x| indexer.get(x.to_owned()))
            .collect::<Vec<_>>();
        for node in &nodes[1..] {
            edges.push(((*node).to_owned(), nodes[0].to_owned()));
        }
        flows.insert(nodes[0].to_owned(), flow);
    }

    let graph = UnGraphMap::<_, ()>::from_edges(edges);
}
