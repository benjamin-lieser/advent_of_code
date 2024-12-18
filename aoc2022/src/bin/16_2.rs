use advent_of_code::*;

static GRAPH: Global<UnGraphMap<u8, ()>> = Global::new();
static FLOWS: Global<Vec<int>> = Global::new();
static MEMO: Global<HashMap<(smallbitset::Set64, u8,u8,u8), int>> = Global::new();

fn max_release(pos1: u8, pos2: u8, time_left: u8, open: smallbitset::Set64) -> int {
    if time_left == 0 {
        return 0;
    }

    if let Some(&result) = MEMO.borrow().get(&(open, pos1, pos2, time_left)) {
        return result;
    }

    let release = FLOWS
        .borrow()
        .iter()
        .enumerate()
        .map(|(idx, flow, )| if open.contains(idx) { flow } else { &0 })
        .sum::<int>();

    let mut max = 0;

    for next1 in GRAPH.borrow().neighbors(pos1) {
        for next2 in GRAPH.borrow().neighbors(pos2) {
            let mut new_open = open.clone();
            if next1 == pos1 {
                if FLOWS.borrow()[next1 as usize] > 0 {
                    new_open = new_open.add(next1 as usize);
                } else {
                    continue;
                }
            }
            if next2 == pos2 {
                if FLOWS.borrow()[next2 as usize] > 0 {
                    new_open = new_open.add(next2 as usize);
                } else {
                    continue;
                }
            }
            let future = max_release(next1, next2, time_left - 1, new_open);
            max = max.max(release + future);
        }
    }

    MEMO.borrow_mut().insert((open, pos1, pos2, time_left), max);
    max
}

fn main() {

    MEMO.set(HashMap::new());

    dbg!(std::mem::size_of::<smallbitset::Set64>());

    let input = get_input(2022, 16);

    let mut indexer = Indexer::new();

    let mut edges = vec![];

    let mut flows = vec![0; input.lines().count()];

    for line in input.lines() {
        let [flow] = get_all_int(&line);
        let nodes = get_all_matches(&line, "[A-Z][A-Z]")
            .iter()
            .map(|&x| indexer.get(x) as u8)
            .collect::<Vec<_>>();
        for node in &nodes {
            edges.push((*node, nodes[0]));
        }
        flows[nodes[0] as usize] = flow;
    }

    FLOWS.set(flows);

    let graph = UnGraphMap::<_, ()>::from_edges(edges);
    GRAPH.set(graph);

    println!(
        "{}",
        max_release(
            indexer.get("AA") as u8,
            indexer.get("AA") as u8,
            26,
            smallbitset::Set64::empty()
        )
    );
}
