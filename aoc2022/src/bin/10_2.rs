use advent_of_code::*;

fn main() {
    let input = get_input(2022, 10);

    let mut cycle = 0;

    let mut x = 1;

    let mut crt = "".to_string();

    let mut draw = | cycle : int, x : int| {
        let pix = if ((cycle - 1) % 40 - x).abs() < 2 {'#'} else {'.'};
        crt.push(pix);
        if cycle % 40 == 0 {
            crt.push('\n');
        }
    };

    for line in input.lines() {
        if line.starts_with("noop") {
            cycle += 1;
            draw(cycle, x);
        } else {
            let [_, num] = split_s(line, "");
            let diff = num.parse::<int>().unwrap();
            cycle += 1;
            draw(cycle, x);
            cycle += 1;
            draw(cycle, x);
            x += diff;
        }
    }


    println!("{}", crt);
}
