use advent_of_code::*;

fn fragment(disk: &mut [int]) -> bool {
    let first_free = disk.iter().position(|&x| x == -1).unwrap();
    let last_block = disk.iter().rposition(|&x| x != -1).unwrap();
    if first_free < last_block {
        disk.swap(first_free as usize, last_block as usize);
        return true;
    }
    false
}

fn main() {
    let input = get_input(2024, 9);

    let mut acc = 0;

    let mut disk = vec![];

    let mut block = true;

    let mut block_id: int = 0;

    for char in input.trim().chars() {
        let digit = char.to_digit(10).unwrap();
        if block {
            for _ in 0..digit {
                disk.push(block_id);
            }
            block_id += 1;
        } else {
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