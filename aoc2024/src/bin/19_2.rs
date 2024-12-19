use advent_of_code::*;

static PATTERN : Global<Vec<String>> = Global::new();

#[memoize::memoize]
fn is_possible(s : String) -> int {
    if s.is_empty() {
        return 1;
    }
    let mut result = 0;
    for pat in PATTERN.borrow().iter() {
        if s.starts_with(pat) {
            result += is_possible(s[pat.len()..].to_owned());
        }
    }
    result
}

fn main() {
    let input = get_input(2024, 19);

    let [pattern, input] = split_empty_line_static(&input);

    let pattern = pattern.split(", ").map(str::to_owned).collect::<Vec<_>>();

    PATTERN.set(pattern);

    let mut acc = 0;

    for line in input.lines() {
        acc += is_possible(line.to_owned());
    }

    println!("{}", acc);
    
}
