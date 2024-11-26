use advent_of_code::*;

#[derive(Debug)]
struct Game {
    runs : Vec<[u64;3]>,
}

const COLORS : [&'static str;3] = ["red", "green", "blue"];

fn parse(s: &str) -> Game {
    let(_, games) = s.split_once(':').unwrap();

    let mut runs = vec![];

    for game in games.split(';') {
        let mut count = [0u64;3];
        for color in game.split(',') {
            let color = color.trim();
            for (idx, c) in COLORS.iter().enumerate() {
                if color.ends_with(c) {
                    let [n, _] = split_s(color, "");
                    count[idx] += n.parse::<u64>().unwrap();
                }
            }
        }
        runs.push(count);
    }

    Game {runs}
}

fn main() {
    let input = get_input(2023, 2);
    let mut acc: u64 = 0;
    for line in input.lines() {
        let game = parse(line);

        let [a,b,c] = game.runs.iter().fold([0u64;3], |acc, e| acc.max_elem(e));

        acc += a * b * c;
        
    }
    println!("{}", acc);
}
