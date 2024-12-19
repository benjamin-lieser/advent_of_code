use advent_of_code::*;

static PATTERN : Global<Vec<String>> = Global::new();

#[memoize::memoize]
fn is_possible(s : String) -> bool {
    if s.is_empty() {
        return true;
    }
    for pat in PATTERN.borrow().iter() {
        if s.starts_with(pat) && is_possible(s[pat.len()..].to_owned()) {
            return true;
        }
    }
    false
}

fn main() {
    let input = get_input(2024, 19);

    let [pattern, input] = split_empty_line_static(&input);

    let pattern = pattern.split(", ").map(str::to_owned).collect::<Vec<_>>();

    PATTERN.set(pattern);

    let mut acc = 0;

    for line in input.lines() {
        if is_possible(line.to_owned()) {
            acc += 1;
        }
    }

    println!("{}", acc);
    
}
