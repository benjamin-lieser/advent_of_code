use advent_of_code::*;

static STATE: Global<HashMap<&str, bool>> = Global::new();
static GATES: Global<HashMap<&str, (Gate, &str, &str)>> = Global::new();

enum Gate {
    And,
    Or,
    Xor,
}

fn eval(wire: &'static str) -> bool {
    if let Some(val) = STATE.borrow().get(wire) {
        return *val;
    }

    let (gate, wire1, wire2) = GATES.borrow().get(wire).unwrap();

    let val1 = eval(wire1);
    let val2 = eval(wire2);

    let res = match gate {
        Gate::And => val1 && val2,
        Gate::Or => val1 || val2,
        Gate::Xor => val1 ^ val2,
    };

    STATE.borrow_mut().insert(wire, res);

    res
}

fn main() {
    let input = Box::leak(get_input(2024, 24).into_boxed_str());

    let [input, gates] = split_empty_line_static(input);

    let mut state = HashMap::new();

    for line in input.lines() {
        let [wire, val] = split_s(line, ": ");
        state.insert(wire, val == "1");
    }

    let mut gates_map = HashMap::new();

    for gate in gates.lines() {
        let [wire1, gate, wire2, _, out] = split_s(gate, " ");
        let gate = match gate {
            "AND" => Gate::And,
            "OR" => Gate::Or,
            "XOR" => Gate::Xor,
            _ => panic!(),
        };
        gates_map.insert(out, (gate, wire1, wire2));
    }

    STATE.set(state);
    GATES.set(gates_map);

    let mut result : int = 0;

    for i in 0.. {
        let output = Box::leak(format!("z{:02}", i).into_boxed_str());
        if !GATES.borrow().contains_key(output) {
            break;
        }
        if eval(output) {
            result += 1 << i;
        }
    }

    println!("{}", result);
}
