use advent_of_code::*;

fn main() {
    let input = get_input(2022, 1);

    let lines: Vec<&str> = input.lines().collect();

    let mut a: Vec<u64> = lines.split(|l| l.is_empty()).map(|a| a.iter().map(|x| x.parse::<u64>().unwrap()).sum::<u64>()).collect();
    
    a.sort_by(|a, b| b.cmp(a));
    
    println!("{}", a[0..3].iter().sum::<u64>());
}
