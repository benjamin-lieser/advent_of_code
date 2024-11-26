use advent_of_code::*;

fn main() {
    let input = get_input(2023, 5);

    let input: Vec<&str> = input.lines().collect();

    let seeds = input[0];
    let (_, seeds) = seeds.so(": ");
    let mut seeds: Vec<u64> = seeds.scast("");
    

    for conversion in input[2..].split(|x| *x == "") {
        let conversion : Vec<[u64;3]> = conversion[1..].iter().map(|x| split(x, "")).collect();
        for seed in seeds.iter_mut() {
            for [d_start, s_start, range] in conversion.iter() {
                if *seed >= *s_start && *seed < *s_start + *range {
                    *seed = *d_start + (*seed - *s_start);
                    break;
                }
            }
        }
    }

    println!("{}", seeds.iter().min().unwrap());
}
