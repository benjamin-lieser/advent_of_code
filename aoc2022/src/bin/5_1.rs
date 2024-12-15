use advent_of_code::*;

fn main() {
    let input = get_input(2022, 5);

    let [crates, commands] = split_empty_line_static(&input);

    let crates : Vec<_> = crates.lines().collect();

    let num_stacks = crates[0].len() / 4 + 1;

    let mut stacks = vec![vec![]; num_stacks];

    for lines in crates.iter().rev().skip(1) {
        for (idx, c) in lines.chars().enumerate() {
            if c != ' ' && c.is_alphabetic() {
                stacks[idx / 4].push(c);
            }
        }
    }

    for command in commands.lines() {
        let [count, from, to] = get_all_pos_int(command);

        for _ in 0..count {
            let c = stacks[from as usize - 1].pop().unwrap();
            stacks[to as usize - 1].push(c);
        }
    }

    for stack in stacks.iter() {
        print!("{}", stack.last().unwrap());
    }

    println!();
}
