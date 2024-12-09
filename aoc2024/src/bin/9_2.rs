use advent_of_code::*;

fn fragment(blocks: &mut [int], free: &mut [int]) -> bool {
    
}

fn main() {
    let input = get_input(2024, 9);

    let mut acc = 0;

    let mut blocks = vec![];
    let mut free = vec![];
    let mut disk = vec![];

    let mut block = true;

    let mut block_id: int = 0;

    for char in input.trim().chars() {
        let digit = char.to_digit(10).unwrap() as int;
        if block {
            for _ in 0..digit {
                disk.push(block_id);
            }
            blocks.push(digit);
            block_id += 1;
        } else {
            free.push((digit, disk.len()));
            for _ in 0..digit {
                disk.push(-1);
            }
        }
        block = !block;
    }

    while fragment(&mut disk) {}

    for (i, &block) in disk.iter().enumerate() {
        if block != -1 {
            acc += block * i as int;
        }
    }

    println!("{}", acc);
}