use std::vec;

use advent_of_code::*;

fn main() {
    let input = get_input(2023, 10);

    let grid: Vec<&str> = input.lines().collect();

    let X = grid[0].len();
    let Y = grid.len();

    let nodes = X * Y;

    let mut graph: Vec<Vec<usize>> = vec![vec![]; nodes];

    let idx = |x: isize, y: isize| {
        let x = x.clamp(0, X as isize - 1);
        let y = y.clamp(0, Y as isize- 1);
        (y * X as isize + x) as usize
    };

    let mut s = 0;

    for (y, line) in grid.iter().enumerate() {
        for (x, pipe) in line.chars().enumerate() {
            let x = x as isize;
            let y = y as isize;
            match pipe {
                '|' => {
                    graph[idx(x, y)].push(idx(x, y - 1));
                    graph[idx(x, y)].push(idx(x, y + 1));
                }
                '-' => {
                    graph[idx(x, y)].push(idx(x - 1, y));
                    graph[idx(x, y)].push(idx(x + 1, y));
                }
                'J' => {
                    graph[idx(x, y)].push(idx(x, y - 1));
                    graph[idx(x, y)].push(idx(x - 1, y));
                }
                'F' => {
                    graph[idx(x, y)].push(idx(x, y + 1));
                    graph[idx(x, y)].push(idx(x + 1, y));
                }
                'L' => {
                    graph[idx(x, y)].push(idx(x, y - 1));
                    graph[idx(x, y)].push(idx(x + 1, y));
                }
                '7' => {
                    graph[idx(x, y)].push(idx(x, y + 1));
                    graph[idx(x, y)].push(idx(x - 1, y));
                }
                'S' => {
                    graph[idx(x, y)].push(usize::MAX);
                    s = idx(x, y);
                }
                '.' => {}
                _ => panic!(),
            }
        }
    }

    //Find some node which points to S
    let mut current = graph.iter().enumerate().find_map(|(idx, next)| if next.contains(&s) {Some(idx)} else {None}).unwrap();
    let mut prev = s;
    let mut count = 1;
    loop {
        if graph[current][0] == usize::MAX {
            break;
        }
        assert!(graph[current].len() == 2);
        let next = graph[current].iter().find(|x| **x != prev).unwrap();
        prev = current;
        current = *next;

        count += 1;
    }

    println!("{}", (count + 1)/2);
}
