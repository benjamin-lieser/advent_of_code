use advent_of_code::*;

static GRAPH: Global<UnGraphMap<usize, ()>> = Global::new();
static FLOWS: Global<Vec<int>> = Global::new();

#[memoize::memoize]
fn max_release(pos: usize, time_left: usize, open: Vec<bool>) -> int {
    if time_left == 0 {
        return 0;
    }

    let release = FLOWS
        .borrow()
        .iter()
        .zip(&open)
        .map(|(flow, open)| if *open { flow } else { &0 })
        .sum::<int>();

    let mut max = 0;

    if open[pos] == false && FLOWS.borrow()[pos] > 0 {
        let mut new_open = open.clone();
        new_open[pos] = true;
        let future = max_release(pos, time_left - 1, new_open);
        max = max.max(release + future);
    }

    for next in GRAPH.borrow().neighbors(pos) {
        let future = max_release(next, time_left - 1, open.clone());
        max = max.max(release + future);
    }
    max
}

fn main() {
    let input = get_input(2022, 16);

    let mut indexer = Indexer::new();

    let mut edges = vec![];

    let mut flows = vec![0; input.lines().count()];

    for line in input.lines() {
        let [flow] = get_all_int(&line);
        let nodes = get_all_matches(&line, "[A-Z][A-Z]")
            .iter()
            .map(|&x| indexer.get(x.to_owned()))
            .collect::<Vec<_>>();
        for node in &nodes[1..] {
            edges.push((*node, nodes[0]));
        }
        flows[nodes[0]] = flow;
    }

    let open = vec![false; flows.len()];
    FLOWS.set(flows);

    let graph = UnGraphMap::<_, ()>::from_edges(edges);
    GRAPH.set(graph);

    println!("{}", max_release(indexer.get("AA".to_string()), 30, open));

}
