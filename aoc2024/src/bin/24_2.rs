use std::vec;

use advent_of_code::*;

fn switch(out1: &str, out2: &str, gates: &str) -> String {
    let mut res = "".to_string();
    for line in gates.lines() {
        let [wire1, gate, wire2, _, out] = split_s(line, " ");

        if out == out1 {
            res.push_str(&format!("{} {} {} -> {}\n", wire1, gate, wire2, out2));
        } else if out == out2 {
            res.push_str(&format!("{} {} {} -> {}\n", wire1, gate, wire2, out1));
        } else {
            res.push_str(line);
            res.push_str("\n");
        }
    }
    res
}

fn main() {
    let input = Box::leak(get_input(2024, 24).into_boxed_str());

    let [_, gates] = split_empty_line_static(input);

    let gates = switch("z12", "djg", gates);
    let gates = switch("mcq", "hjm", &gates);
    let gates = switch("z37", "dsd", &gates);
    let gates = switch("z19", "sbg", &gates);

    let mut first_xor = vec![""; 45];
    let mut first_and = vec![""; 45];
    let mut second_and = vec![""; 45];

    let mut new_gates = "".to_string();
    for line in gates.lines() {
        let [wire1, gate, wire2, _, out] = split_s(line, " ");

        if gate == "XOR" && wire1[1..] == wire2[1..] {
            first_xor[wire1[1..].parse::<usize>().unwrap()] = out;
            continue;
        }
        new_gates.push_str(line);
        new_gates.push_str("\n");
    }

    for i in 0..=44 {
        new_gates = new_gates.replace(first_xor[i], &format!("_xor{}", i));
    }

    let mut gates2 = "".to_string();

    for line in new_gates.lines() {
        let [wire1, gate, wire2, _, out] = split_s(line, " ");

        if gate == "AND" && wire1[1..] == wire2[1..] {
            first_and[wire1[1..].parse::<usize>().unwrap()] = out;
            continue;
        }
        gates2.push_str(line);
        gates2.push_str("\n");
    }

    for i in 0..=44 {
        gates2 = gates2.replace(first_and[i], &format!("_and{}", i));
    }

    dbg!(first_xor);
    dbg!(first_and);

    println!("{}", gates2);

    let mut gates3 = "".to_string();

    for line in gates2.lines() {
        let [wire1, gate, wire2, _, out] = split_s(line, " ");

        if gate == "AND" {
            if wire1.starts_with("_xor") {
                second_and[wire1[4..].parse::<usize>().unwrap()] = out;
            } else if wire2.starts_with("_xor") {
                second_and[wire2[4..].parse::<usize>().unwrap()] = out;
            } else {
                gates3.push_str(line);
                gates3.push_str("\n");
            }
            continue;
        }
        gates3.push_str(line);
        gates3.push_str("\n");
    }

    dbg!(&second_and);

    for i in 0..=44 {
        if second_and[i] == "" {
            continue;
        }
        gates3 = gates3.replace(second_and[i], &format!("_2and{}", i));
    }

    println!("{}", gates3);

    let mut carries = vec![""; 45];

    let mut gates4 = "".to_string();

    for line in gates3.lines() {
        let [wire1, gate, _, _, out] = split_s(line, " ");
        if gate == "OR" {
            if wire1.starts_with("_and") {
                carries[wire1[4..].parse::<usize>().unwrap()] = out;
            } else if wire1.starts_with("_2and") {
                carries[wire1[5..].parse::<usize>().unwrap()] = out;
            }
            continue;
        }
        gates4.push_str(line);
        gates4.push_str("\n");
    }

    dbg!(&carries);

    for i in 0..=44 {
        if carries[i] == "" {
            continue;
        }
        gates4 = gates4.replace(carries[i], &format!("_carry{}", i));
    }

    println!("{}", gates4);
}
