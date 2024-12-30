use advent_of_code::*;

fn main() {
    let input = get_input(2024, 13);

    let mut acc = 0;

    let add = 10000000000000isize;

    for input in split_empty_line(&input) {
        let [a,b,prize] = split_s(&input, "\n");

        let double_number = regex::Regex::new(r"(\d+), Y[\+|=](\d+)").unwrap();
        let ax : int = double_number.captures(&a).unwrap()[1].parse().unwrap();
        let bx : int = double_number.captures(&b).unwrap()[1].parse().unwrap();
        let prize_x : int= double_number.captures(&prize).unwrap()[1].parse::<int>().unwrap() + add;
        let ay : int= double_number.captures(&a).unwrap()[2].parse().unwrap();
        let by : int = double_number.captures(&b).unwrap()[2].parse().unwrap();
        let prize_y :int = double_number.captures(&prize).unwrap()[2].parse::<int>().unwrap() + add;

        let matrix = [[ax, bx], [ay, by]];
        let sol = [prize_x, prize_y];

        if let Some([ai,bi]) = lin_sol(matrix, sol) {
            let ai = ai.round() as int;
            let bi = bi.round() as int;
            if ai >= 0 && bi >= 0 && ax * ai + bx * bi == prize_x && ay * ai + by * bi == prize_y {
                acc += 3 * ai + bi;
            }
        } else {
            println!("No solution"); // Does not happen in the input :D
        }
    }

    println!("{}", acc);

}