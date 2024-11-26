use advent_of_code::*;

fn main() {
    let input = get_input(2023, 3);

    let mut symbols = HashMap::<(isize,isize),Vec<u64>>::new();

    //Symbol pass
    for (y, line) in input.lines().enumerate() {
        for x in line.match_indices(|c: char|  c == '*').map(|(i, _)| i) {
            symbols.insert((y as isize, x as isize), vec![]);
        }
    }

    let mut acc = 0u64;

    for (y, line) in input.lines().enumerate() {
        
        let number = regex::Regex::new(r"[\d]+").unwrap();

        for mat in number.find_iter(&line) {
            for of in -1..=1 {
                for x in mat.start() as isize -1..mat.end() as isize+1 {
                    if let Some(v) = symbols.get_mut(&(y as isize + of, x as isize)) {
                        v.push(mat.as_str().parse().unwrap());
                    }
                }
            }
        }
        
    }
    
    for g in symbols.values() {
        if g.len() == 2 {
            acc += g[0] * g[1];
        }
    }

    println!("{}", acc);
}