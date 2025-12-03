use advent_of_code::*;

fn best_digit(d : &[i64], remaining : usize) -> (&[i64], i64) {
    let valid_range = &d[..d.len()-remaining];

    let max = valid_range.iter().max().unwrap();
    let pos = valid_range.iter().position(|x| x == max).unwrap();

    (&d[pos+1..], *max)
    
}

fn main() {
    let input = get_input(2025, 3);

    let mut counter = 0;

    for line in input.lines() {
        let digits = line
            .chars()
            .map(|c| c.to_digit(10).unwrap() as i64)
            .collect::<Vec<_>>();

        let mut digits = &digits[..];
        
        for i in (0..=11).rev() {
            let (rest, best) = best_digit(&digits, i);
            counter += best * 10_i64.pow(i as u32);
            digits = rest;
        }

    }

    println!("{}", counter);
}
