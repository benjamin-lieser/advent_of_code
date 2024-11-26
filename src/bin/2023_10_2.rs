use colored::Colorize;
use std::vec;

use advent_of_code::*;

fn main() {
    let input = get_input_aoc(10);

    let grid: Vec<&str> = input.lines().collect();

    let X = grid[0].len();
    let Y = grid.len();

    let nodes = X * Y;

    let mut graph: Vec<Vec<usize>> = vec![vec![]; nodes];

    let idx = |x: isize, y: isize| {
        let x = x.clamp(0, X as isize - 1);
        let y = y.clamp(0, Y as isize - 1);
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

    let mut path: Vec<usize> = vec![];
    //Find some node which points to S
    let mut current = graph
        .iter()
        .enumerate()
        .find_map(|(idx, next)| if next.contains(&s) { Some(idx) } else { None })
        .unwrap();
    let mut prev = s;

    path.push(prev);
    path.push(current);
    let mut count = 1;
    loop {
        if graph[current][0] == usize::MAX {
            break;
        }
        assert!(graph[current].len() == 2);
        let next = graph[current].iter().find(|x| **x != prev).unwrap();
        prev = current;
        current = *next;
        path.push(*next);
        count += 1;
    }

    let mut is_path = vec![vec![false; X]; Y];

    for y in 0..Y as isize {
        for x in 0..X as isize {
            if path.contains(&idx(x, y)) {
                print!(
                    "{}",
                    (grid[y as usize].as_bytes()[x as usize] as char)
                        .to_string()
                        .red()
                );
                is_path[y as usize][x as usize] = true;
            } else {
                print!("{}", grid[y as usize].as_bytes()[x as usize] as char);
            }
        }
        println!("")
    }

    let is_inside = |x: usize, y: usize| {
        let left = &grid[y][x..];
        let path_left = &is_path[y][x..];

        if path_left[0] { //Its a path
            return false;
        }

        let mut stack = vec![];

        let mut parity = 0;

        for (idx, c) in left.chars().enumerate() {
            if !path_left[idx] {
                continue;
            }
            match c {
                'L' => stack.push('L'),
                'F' => stack.push('F'),
                '-' => {}
                '|' | 'S' => parity += 1, // Secret knowledte that S is a | in this input data
                'J' => {
                    if stack.last().unwrap() == &'F' {
                        parity += 1
                    }
                }
                '7' => {
                    if stack.last().unwrap() == &'L' {
                        parity += 1
                    }
                }
                _ => panic!(),
            }
        }

        return parity % 2 == 1
    };

    let mut in_counter = 0;

    for y in 0..Y {
        for x in 0..X {
            if is_inside(x, y) {
                in_counter += 1;
            }
        }
    }

    println!("{}", in_counter);
}
