use advent_of_code::*;

fn follow(tail: Step, head: Step) -> Step {
    if (head - tail).max_norm() < 2 {
        return tail;
    }

    let mut diff = head - tail;

    if diff.0 == 0 {
        diff.1 = diff.1.signum();
    } else if diff.1 == 0 {
        diff.0 = diff.0.signum();
    } else if diff.0.abs() == 2 && diff.1.abs() == 2 {
        diff.0 = diff.0.signum();
        diff.1 = diff.1.signum();
    } else if diff.0.abs() == 2 {
        diff.0 = diff.0.signum();
    } else if diff.1.abs() == 2 {
        diff.1 = diff.1.signum();
    } else {
        panic!("Invalid diff: {:?}", diff);
    }

    tail + diff
}

fn main() {
    let input = get_input(2022, 9);

    let mut touch = HashSet::new();

    let mut knots = vec![Step(0, 0);10];

    for line in input.lines() {
        let [dir, num] = split_s(line, "");
        let num = num.parse::<int>().unwrap();
        let dir = dir.parse::<Dir>().unwrap();

        for _ in 0..num {
            knots[0] = knots[0] + dir;
            for i in 1..10 {
                knots[i] = follow(knots[i], knots[i - 1]);
            }
            touch.insert(knots[9]);
        }
    }

    println!("{}", touch.len());
}
