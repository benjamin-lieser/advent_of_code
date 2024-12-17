use advent_of_code::*;

static GRAPH: Global<UnGraphMap<usize, ()>> = Global::new();
static FLOWS: Global<Vec<int>> = Global::new();

#[memoize::memoize]
fn max_release(pos1: usize, pos2: usize, time_left: usize, open: Vec<bool>) -> int {
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

    for next1 in GRAPH.borrow().neighbors(pos1) {
        for next2 in GRAPH.borrow().neighbors(pos2) {
            let mut new_open = open.clone();
            if next1 == pos1 {
                if FLOWS.borrow()[next1] > 0 {
                    new_open[next1] = true;
                }
            }
            if next2 == pos2 {
                if FLOWS.borrow()[next2] > 0 {
                    new_open[next2] = true;
                }
            }
            let future = max_release(next1, next2, time_left - 1, new_open);
            max = max.max(release + future);
        }
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
            .map(|&x| indexer.get(x))
            .collect::<Vec<_>>();
        for node in &nodes {
            edges.push((*node, nodes[0]));
        }
        flows[nodes[0]] = flow;
    }

    let open = vec![false; flows.len()];
    FLOWS.set(flows);

    let graph = UnGraphMap::<_, ()>::from_edges(edges);
    GRAPH.set(graph);

    println!(
        "{}",
        max_release(
            indexer.get("AA"),
            indexer.get("AA"),
            26,
            open
        )
    );
}
