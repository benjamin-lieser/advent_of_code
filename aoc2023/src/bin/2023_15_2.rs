use advent_of_code::*;

fn hash(com: &[u8]) -> usize {
    com.iter()
        .fold(0u8, |acc, new| acc.wrapping_add(*new).wrapping_mul(17)) as usize
}

fn main() {
    let input = get_input_aoc(15);

    let mut map: [Vec<(&str, u8)>; 256] = std::array::from_fn(|_| Vec::new());

    for comand in input.trim().split_terminator(',') {
        if let Some((name, _)) = comand.split_once('-') {
            map[hash(name.as_bytes())].retain(|(x, _)| *x != name);
        }

        if let Some((name, focal)) = comand.split_once('=') {
            let focal = focal.parse::<u8>().unwrap();

            let r#box = &mut map[hash(name.as_bytes())];

            if let Some((_, f)) = r#box.iter_mut().find(|(n, _)| *n == name) {
                *f = focal;
            } else {
                r#box.push((name, focal));
            }
        }
    }

    let mut acc = 0u64;
    for (idx, b) in map.iter().enumerate() {
        for (jdx, &(_, f)) in b.iter().enumerate() {
            acc += (idx + 1) as u64 * (jdx + 1) as u64 * f as u64;
        }
    }

    println!("{}", acc);
}
