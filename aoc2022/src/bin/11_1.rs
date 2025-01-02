use advent_of_code::*;

#[derive(Debug, Clone)]
struct Monkey {
    items: Vec<int>,
    test: int,
    if_false: int,
    if_true: int,
    operation: String,
}

fn monkey_run(monkeys: &mut [Monkey], monkey_id: int, counter: &mut [int]) {
    let monkey = monkeys[monkey_id as usize].clone();
    counter[monkey_id as usize] += monkey.items.len() as int;

    for &item in &monkey.items {
        let old = item as f64;
        let expr: meval::Expr = monkey.operation.parse().unwrap();
        let func = expr.bind("old").unwrap();
        let new = func(old).round() as int;

        let new = new / 3;

        if new % monkey.test == 0 {
            monkeys[monkey.if_true as usize].items.push(new);
        } else {
            monkeys[monkey.if_false as usize].items.push(new);
        }
    }

    monkeys[monkey_id as usize].items.clear();
}

fn main() {
    let input = get_input(2022, 11);

    let mut monkeys = Vec::new();

    for monkey_str in split_empty_line(&input) {
        let lines = monkey_str.lines().collect::<Vec<&str>>();
        let items = get_all_int_dyn(lines[1]);
        let [test] = get_all_int(lines[3]);
        let [if_true] = get_all_int(lines[4]);
        let [if_false] = get_all_int(lines[5]);
        let (_, operation) = lines[2].split_once("=").unwrap();
        monkeys.push(Monkey {
            items,
            test,
            if_false,
            if_true,
            operation: operation.to_string(),
        });
    }

    let mut counter = vec![0; monkeys.len()];
    for _ in 0..20 {
        for i in 0..monkeys.len() {
            monkey_run(&mut monkeys, i as int, &mut counter);
        }
    }

    counter.sort();

    println!("{:?}", counter.iter().rev().take(2).product::<int>());

}
