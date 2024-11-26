use advent_of_code::*;

#[derive(Debug)]
struct Game {
    runs : Vec<[u64;3]>,
    id : u64
}

const COLORS : [&'static str;3] = ["red", "green", "blue"];

fn parse(s: &str) -> Game {
    let(id, games) = s.split_once(':').unwrap();

    let [_, id] = split_s(id, "");
    let id = id.parse::<u64>().unwrap();

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

    Game {runs, id}
}

fn main() {
    let input = get_input(2023, 2);
    let mut acc: u64 = 0;
    for line in input.lines() {
        let game = parse(line);

        if game.runs.iter().all(|c| c[0] <= 12 && c[1] <= 13 && c[2] <= 14) {
            acc += game.id;
        }
    }
    println!("{}", acc);
}
