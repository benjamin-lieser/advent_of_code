use advent_of_code::*;

fn fragment(disk: &mut [int], file_id: int) {
    let first_file = disk.iter().position(|&x| x == file_id).unwrap();
    let last_file = disk.iter().rposition(|&x| x == file_id).unwrap();

    let file_size = last_file as int - first_file as int + 1;

    let mut free_begin = 0;
    let mut free_length = 0;

    for i in 0..disk.len() {
        if disk[i] == -1 {
            free_length += 1;
        } else {
            free_length = 0;
            free_begin = i + 1;
        }
        if free_length == file_size {
            break;
        }
    }

    if free_length >= file_size && free_begin < first_file {
        for i in 0..file_size {
            disk.swap(
                first_file as usize + i as usize,
                free_begin as usize + i as usize,
            );
        }
    }
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

    for i in (0..block_id).rev() {
        fragment(&mut disk, i as int);
    }

    for (i, &block) in disk.iter().enumerate() {
        if block != -1 {
            acc += block * i as int;
        }
    }

    println!("{}", acc);
}
