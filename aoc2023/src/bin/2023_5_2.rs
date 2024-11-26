use advent_of_code::*;

fn main() {
    let input = get_input(2023, 5);

    let input: Vec<&str> = input.lines().collect();

    let seeds: &str = &input[0];
    let (_, seeds) = seeds.so(": ");
    let seeds: Vec<u64> = seeds.scast("");

    let seeds = seeds
        .chunks_exact(2)
        .map(|x| (x[0]..(x[0] + x[1])))
        .flatten();

    let conversions: Vec<Vec<[u64; 3]>> = input[2..]
        .split(|x| *x == "")
        .map(|x| x[1..].iter().map(|x| split(x, "")).collect())
        .collect();

    let mut current_min = u64::MAX;

    for mut seed in seeds {
        for conversion in conversions.iter() {
            for [d_start, s_start, range] in conversion.iter() {
                if seed >= *s_start && seed < *s_start + *range {
                    seed = *d_start + (seed - *s_start);
                    break;
                }
            }
        }
        current_min = current_min.min(seed);
    }

    println!("{}", current_min);
}
