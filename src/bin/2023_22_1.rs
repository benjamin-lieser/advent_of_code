use advent_of_code::*;

#[derive(Debug, Clone, PartialEq, Eq)]
struct Brick {
    start: [usize; 3],
    end: [usize; 3],
    ori: usize,
}

impl Brick {
    fn new(s: &str) -> Self {
        let (start, end) = s.split_once('~').unwrap();
        let start = split_num_at(start, ',');
        let end = split_num_at(end, ',');

        let mut ori = 0;

        for i in 0..3 {
            if start[i] != end[i] {
                ori = i;
                break;
            }
        }

        Brick { start, end, ori }
    }
}

fn main() {
    let input = get_input_aoc(22);

    //let input = std::fs::read_to_string("data/2023_22").unwrap();

    let mut brick = vec![vec![vec![false; 1000]; 10]; 10];

    for x in 0..10 {
        for y in 0..10 {
            brick[x][y][0] = true;
        }
    }

    let mut bricks: Vec<Brick> = input.lines().map(Brick::new).collect();

    for b in bricks.iter() {
        for x in b.start[0]..=b.end[0] {
            for y in b.start[1]..=b.end[1] {
                for z in b.start[2]..=b.end[2] {
                    brick[x][y][z] = true;
                }
            }
        }
    }

    let fall = |b: &mut Brick, brick: &mut Vec<Vec<Vec<bool>>>| {
        if b.ori == 2 {
            if brick[b.start[0]][b.start[1]][b.start[2] - 1] == false {
                brick[b.start[0]][b.start[1]][b.start[2] - 1] = true;
                brick[b.start[0]][b.start[1]][b.end[2]] = false;
                b.start[2] -= 1;
                b.end[2] -= 1;
                return true;
            } else {
                return false;
            }
        } else {
            let mut can_fall = true;
            for x in b.start[0]..=b.end[0] {
                for y in b.start[1]..=b.end[1] {
                    for z in b.start[2]..=b.end[2] {
                        if brick[x][y][z - 1] == true {
                            can_fall = false;
                        }
                    }
                }
            }

            if can_fall {
                for x in b.start[0]..=b.end[0] {
                    for y in b.start[1]..=b.end[1] {
                        for z in b.start[2]..=b.end[2] {
                            brick[x][y][z] = false;
                            brick[x][y][z - 1] = true;
                        }
                    }
                }
                b.start[2] -= 1;
                b.end[2] -= 1;
                return true;
            } else {
                return false;
            }
        }
    };

    loop {
        if bricks
            .iter_mut()
            .map(|b| fall(b, &mut brick))
            .all(|x| x == false)
        {
            break;
        }
    }

    let mut acc = 0;

    for b in bricks.iter() {
        let mut copy = brick.clone();

        for x in b.start[0]..=b.end[0] {
            for y in b.start[1]..=b.end[1] {
                for z in b.start[2]..=b.end[2] {
                    copy[x][y][z] = false;
                }
            }
        }
        let mut brick_c: Vec<(Brick, bool)> = bricks.iter().map(|b| (b.clone(), false)).collect();

        brick_c.retain(|c| c.0 != *b);

        loop {
            if brick_c
                .iter_mut()
                .map(|(b, has_fallen)| {
                    let f = fall(b, &mut copy);
                    *has_fallen = *has_fallen | f;
                    f
                })
                .all(|x| x == false)
            {
                break;
            }
        }

        acc += brick_c.iter().filter(|(_, fallen)| *fallen).count();
    }

    println!("{}", acc)
}
