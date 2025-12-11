use advent_of_code::*;
use petgraph::prelude::*;

fn main() {
    let input = get_input(2025, 10);

    
    let mut counter = 0;

    for line in input.lines() {
        let parts = line.split_ascii_whitespace().collect::<Vec<_>>();
        let config = parts[0][1..parts[0].len()-1].to_string();
        let buttons = &parts[1..parts.len()-1];

        let buttons = buttons.iter().map(|b| {
            let inner = &b[1..b.len()-1];
            let numbers = inner.split(',').map(|x| x.parse::<int>().unwrap()).collect::<Vec<_>>();
            numbers
        }
        ).collect::<Vec<_>>();

        let mut graph = UnGraphMap::new();

        let n = config.len();

        for pattern in 0..(1 << n) {
            let pattern = int_to_bits(pattern, n);
            for button in &buttons {
                let mut new_pattern = pattern.clone();
                for &idx in button {
                    new_pattern[idx as usize] = !new_pattern[idx as usize];
                }
                graph.add_edge(bits_to_int(&pattern), bits_to_int(&new_pattern), 1u64);
            }
        }

        let start = config.chars().map(|c| c == '#').collect::<Vec<_>>();
        
        let dists = petgraph::algo::dijkstra(&graph, bits_to_int(&start), None, |e| *e.weight());

        counter += dists.get(&0).unwrap();

    }

    println!("{}", counter);
}
