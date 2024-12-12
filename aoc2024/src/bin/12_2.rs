use advent_of_code::*;

fn main() {
    let input = get_input(2024, 12);

    let mut acc = 0;

    let grid = Grid::from_str(&input);

    let (clusters, cluster_num) = clusters(&grid);

    for i in 0..cluster_num {
        let area = clusters.count(i);

        let mut sides = 0;

        let is_perimeter_left = |pos| {
            let l = clusters.get(pos + Dir::Left);
            let r = clusters.get(pos);
            if (r == Some(i) || l == Some(i)) && r != l {
                if r == Some(i) {
                    return 1;
                } else {
                    return 2;
                }
            }
            0
        };

        let is_perimeter_up = |pos| {
            let l = clusters.get(pos + Dir::Up);
            let r = clusters.get(pos);
            if (r == Some(i) || l == Some(i)) && r != l {
                if r == Some(i) {
                    return 1;
                } else {
                    return 2;
                }
            }
            0
        };

        for line in 0..=grid.c() {
            let mut peri = 0;
            for row in 0..grid.r() {
                let status = is_perimeter_left((line, row));
                if status != peri && status != 0 {
                    sides += 1;
                }
                peri = status;
            }
        }

        for line in 0..=grid.r() {
            let mut peri = 0;
            for col in 0..grid.c() {
                let status = is_perimeter_up((col, line));
                if status != peri && status != 0 {
                    sides += 1;
                }
                peri = status;
            }
        }
        //dbg!(sides, area);
        acc += area * sides;
    }

    println!("{}", acc);
}
