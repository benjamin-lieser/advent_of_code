use advent_of_code::*;

struct CPU {
    ic: int,
    a: int,
    b: int,
    c: int,
    instructions: Vec<int>,
}

fn combo(cpu: &CPU, val: int) -> int {
    match val {
        0..=3 => val,
        4 => cpu.a,
        5 => cpu.b,
        6 => cpu.c,
        _ => panic!("Invalid combo value"),
    }
}

fn div(num: int, denom: int) -> int {
    if denom > 63 {
        return 0;
    }
    num / (1 << denom)
}

fn step(cpu: &mut CPU) -> Option<Option<int>> {
    if cpu.ic as usize >= cpu.instructions.len() {
        return None;
    }
    let instruction = cpu.instructions[cpu.ic as usize];
    let operant = cpu.instructions[cpu.ic as usize + 1];

    let result = match instruction {
        0 => {
            cpu.a = div(cpu.a, combo(cpu, operant));
            Some(None)
        }
        1 => {
            cpu.b = cpu.b ^ operant;
            Some(None)
        }
        2 => {
            cpu.b = combo(cpu, operant) % 8;
            Some(None)
        }
        3 => {
            if cpu.a != 0 {
                cpu.ic = operant;
                return Some(None);
            }
            Some(None)
        }
        4 => {
            cpu.b = cpu.b ^ cpu.c;
            Some(None)
        }
        5 => Some(Some(combo(cpu, operant) % 8)),
        6 => {
            cpu.b = div(cpu.a, combo(cpu, operant));
            Some(None)
        }
        7 => {
            cpu.c = div(cpu.a, combo(cpu, operant));
            Some(None)
        }
        _ => panic!("Invalid instruction"),
    };

    cpu.ic += 2;

    result
}

fn output(cpu: &mut CPU) -> Vec<int> {
    let mut output = vec![];
    loop {
        match step(cpu) {
            Some(Some(val)) => {
                output.push(val);
            }
            Some(None) => (),
            None => return output,
        }
    }
}

fn main() {
    let input = get_input(2024, 17);

    let lines: Vec<_> = input.lines().collect();

    let (_, instructions) = lines[4].so(": ");

    let instructions: Vec<int> = instructions
        .split(",")
        .map(|x| x.parse().unwrap())
        .collect();

    let mut a_bits = [0;16];

    let mut i = 0;

    while i < 16 {
        let mut found = false;
        while a_bits[i] < 8 {
            let mut cpu = CPU {
                ic: 0,
                a: polynomail_eval_int(&a_bits, 8),
                b: 0,
                c: 0,
                instructions: instructions.clone(),
            };
            let output = output(&mut cpu);
            if output.len() == 16 && output[15-i] == instructions[15-i] {
                found = true;
                break;
            }
            a_bits[i] += 1;
        }
        if !found {
            a_bits[i] = 0;
            i -= 1;
            a_bits[i] += 1;
            continue;
        }
        i += 1;
    }    

    let a = polynomail_eval_int(&a_bits, 8);

    let mut cpu = CPU {
        ic: 0,
        a: a,
        b: 0,
        c: 0,
        instructions: instructions.clone(),
    };

    println!("{:?}", a_bits);

    println!("{:?}", output(&mut cpu));


    println!("{}", polynomail_eval_int(&a_bits, 8));

    

}
