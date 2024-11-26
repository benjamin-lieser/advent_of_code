use advent_of_code::*;

fn is_near(symbols : &HashSet<(isize, isize)>, pos : (isize, isize)) -> bool {
    for i in -1..=1 {
        for j in -1..=1 {
            if symbols.contains(&(pos.0 + i, pos.1 + j)) {
                return true;
            }
        }
    }
    false
}

fn main() {
    let input = get_input(2023, 3);

    let mut symbols = HashSet::<(isize,isize)>::new();

    //Symbol pass
    for (y, line) in input.lines().enumerate() {
        for x in line.match_indices(|c: char| !c.is_ascii_digit() && c != '.').map(|(i, _)| i) {
            symbols.insert((y as isize, x as isize));
        }
    }

    let mut acc = 0u64;

    for (y, line) in input.lines().enumerate() {
        
        let number = regex::Regex::new(r"[\d]+").unwrap();

        for mat in number.find_iter(&line) {
            if (mat.start()..mat.end()).any(|x| is_near(&symbols, (y as isize, x as isize))) {
                acc += mat.as_str().parse::<u64>().unwrap();
            }
        }
        
    }

    println!("{}", acc);
}